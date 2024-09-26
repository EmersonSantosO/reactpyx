use dashmap::DashMap;
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
    children: Vec<VNode>,
    is_critical: bool,
    cache: Arc<DashMap<String, CacheEntry>>,
    cache_duration: Duration,
}

#[pymethods]
impl VNode {
    #[new]
    pub fn new(
        tag: String,
        props: HashMap<String, PyObject>,
        children: Vec<VNode>,
        is_critical: bool,
        cache_duration_secs: u64,
    ) -> Self {
        VNode {
            tag,
            props,
            children,
            is_critical,
            cache: Arc::new(DashMap::new()),
            cache_duration: Duration::from_secs(cache_duration_secs),
        }
    }

    pub fn render_vnode(&self, py: Python) -> PyResult<String> {
        let cache_key = format!("{:?}{:?}", self.tag, self.props);

        if let Some(entry) = self.cache.get(&cache_key) {
            if entry.timestamp.elapsed() < self.cache_duration {
                return Ok(entry.value.clone());
            }
        }

        let mut props_str = String::new();
        for (k, v) in &self.props {
            let value = v.extract::<String>(py)?;
            props_str.push_str(&format!(" {}=\"{}\"", k, value));
        }

        let mut children_str = String::new();
        for child in &self.children {
            children_str.push_str(&child.render_vnode(py)?);
        }

        let rendered = format!("<{}{}>{}</{}>", self.tag, props_str, children_str, self.tag);

        self.cache.insert(
            cache_key,
            CacheEntry {
                value: rendered.clone(),
                timestamp: Instant::now(),
            },
        );

        Ok(rendered)
    }
}
