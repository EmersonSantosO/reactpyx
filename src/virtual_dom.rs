use pyo3::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
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
    cache: Arc<Mutex<HashMap<String, CacheEntry>>>, // Cambiado a `Mutex`
    cache_duration: Duration,
}

#[pymethods]
impl VNode {
    pub fn clean_cache(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.retain(|_, entry| entry.timestamp.elapsed() < self.cache_duration);
    }

    pub fn set_cache_duration(&mut self, duration_secs: u64) {
        self.cache_duration = Duration::from_secs(duration_secs);
    }

    pub fn render_vnode(&self, py: Python) -> PyResult<String> {
        let cache_key = format!("{:?}{:?}", self.tag, self.props);
        self.clean_cache();

        if !self.is_critical {
            let cache = self.cache.lock().unwrap();
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
                let child_vnode = child.bind(py).borrow();
                child_vnode.render_vnode(py)
            })
            .collect::<PyResult<String>>()?;

        let rendered = format!("<{}{}>{}</{}>", self.tag, props_str, children_str, self.tag);

        if !self.is_critical {
            let mut cache = self.cache.lock().unwrap();
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
