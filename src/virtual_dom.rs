// core_reactpyx/src/virtual_dom.rs

use dashmap::DashMap;
use pyo3::prelude::*;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct CacheEntry {
    value: String,
    timestamp: Instant,
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct VNode {
    pub tag: String,
    pub props: HashMap<String, PyObject>,
    pub children: Vec<VNode>,
    pub is_critical: bool,
    pub cache: Arc<DashMap<String, CacheEntry>>,
    pub cache_duration: Duration,
}

#[pymethods]
impl VNode {
    #[new]
    pub fn new(
        tag: &str,
        props: HashMap<String, PyObject>,
        children: Vec<VNode>,
        is_critical: bool,
        cache_duration_secs: u64,
    ) -> Self {
        VNode {
            tag: tag.to_string(),
            props,
            children,
            is_critical,
            cache: Arc::new(DashMap::new()),
            cache_duration: Duration::from_secs(cache_duration_secs),
        }
    }

    pub fn render_vnode(&self, py: Python<'_>) -> PyResult<String> {
        let cache_key = format!("{:?}{:?}", self.props, self.children);

        if let Some(cached_value) = self.cache.get(&cache_key) {
            if self.is_cache_valid(&cached_value) {
                return Ok(cached_value.value.clone());
            }
        }

        let props = self
            .props
            .iter()
            .map(|(k, v)| {
                if let Ok(s) = v.extract::<String>(py) {
                    Ok(format!("{}=\"{}\"", k, s))
                } else if let Ok(b) = v.extract::<bool>(py) {
                    if b {
                        Ok(format!("{}", k))
                    } else {
                        Ok(String::new())
                    }
                } else {
                    // Manejar otros tipos de datos o propagar un error
                    Err(pyo3::exceptions::PyTypeError::new_err(format!(
                        "Tipo de dato no soportado para la propiedad '{}'",
                        k
                    )))
                }
            })
            .collect::<PyResult<Vec<_>>>()?; // Manejar errores en la colección

        let children = self
            .children
            .iter()
            .map(|child| child.render_vnode(py))
            .collect::<PyResult<Vec<_>>>()?; // Manejar errores en la colección

        let rendered = format!(
            "<{} {}>{}</{}>",
            self.tag,
            props.join(" "),
            children.join(""),
            self.tag
        );

        self.cache.insert(
            cache_key,
            CacheEntry {
                value: rendered.clone(),
                timestamp: Instant::now(),
            },
        );

        Ok(rendered)
    }

    fn is_cache_valid(&self, entry: &CacheEntry) -> bool {
        entry.timestamp.elapsed() < self.cache_duration
    }
}
// core_reactpyx/src/virtual_dom.rs

#[cfg(test)]
mod tests {
    use super::*;
    use pyo3::{types::PyDict, Python};

    #[test]
    fn test_render_vnode() {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let mut props = HashMap::new();
        props.insert("id".to_string(), "my-div".to_object(py));
        props.insert("class".to_string(), "container".to_object(py));
        props.insert("data-attr".to_string(), 123.to_object(py));
        props.insert("hidden".to_string(), true.to_object(py));

        let vnode = VNode::new(
            "div",
            props,
            vec![
                VNode::new("p", HashMap::new(), vec![], true, 60),
                VNode::new("span", HashMap::new(), vec![], true, 60),
            ],
            true,
            60,
        );

        let expected_output = r#"<div id="my-div" class="container" data-attr="123" hidden><p></p><span></span></div>"#;
        let actual_output = vnode.render_vnode(py).unwrap();

        assert_eq!(actual_output, expected_output);
    }

    // ... [otras pruebas para render_vnode con diferentes tipos de datos en las propiedades] ...
}
