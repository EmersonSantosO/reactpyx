use parking_lot::RwLock;
use pyo3::prelude::*;
use std::collections::HashMap;
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
    tag: String,
    props: HashMap<String, PyObject>,
    children: Vec<Py<VNode>>,
    is_critical: bool,
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    cache_duration: Duration,
}

#[pymethods]
impl VNode {
    #[new]
    #[pyo3(signature = (tag, props, children, is_critical, cache_duration_secs = 60))]
    pub fn new(
        tag: String,
        props: HashMap<String, PyObject>,
        children: Vec<Py<VNode>>,
        is_critical: bool,
        cache_duration_secs: u64,
    ) -> Self {
        VNode {
            tag,
            props,
            children,
            is_critical,
            cache: Arc::new(RwLock::new(HashMap::new())),
            cache_duration: Duration::from_secs(cache_duration_secs),
        }
    }

    pub fn render_vnode(&self, py: Python) -> PyResult<String> {
        let cache_key = format!("{:?}{:?}", self.tag, self.props);

        // Verifica si el nodo es estático y usa el cache solo si es necesario
        if !self.is_critical {
            let cache = self.cache.read();
            if let Some(entry) = cache.get(&cache_key) {
                if entry.timestamp.elapsed() < self.cache_duration {
                    return Ok(entry.value.clone());
                }
            }
        }

        let props_str = self
            .props
            .iter()
            .map(|(k, v)| Ok(format!(" {}=\"{}\"", k, v.extract::<String>(py)?)))
            .collect::<PyResult<String>>()?;

        let children_str = self
            .children
            .iter()
            .map(|child| {
                let child_vnode = child.as_ref(py).borrow();
                child_vnode.render_vnode(py)
            })
            .collect::<PyResult<String>>()?;

        let rendered = format!("<{}{}>{}</{}>", self.tag, props_str, children_str, self.tag);

        // Cache solo si no es crítico
        if !self.is_critical {
            let mut cache = self.cache.write();
            cache.insert(
                cache_key,
                CacheEntry {
                    value: rendered.clone(),
                    timestamp: Instant::now(),
                },
            );
        }

        Ok(rendered)
    }
}
