use pyo3::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
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
    pub props: HashMap<String, String>,
    pub children: Vec<VNode>,
    pub is_critical: bool, // Nuevo campo para reconciliación por fases
    pub cache: Arc<RwLock<HashMap<String, CacheEntry>>>,
    pub cache_duration: Duration,
}

#[pymethods]
impl VNode {
    #[new]
    pub fn new(
        tag: &str,
        props: HashMap<String, String>,
        children: Vec<VNode>,
        is_critical: bool,
        cache_duration_secs: u64,
    ) -> Self {
        VNode {
            tag: tag.to_string(),
            props,
            children,
            is_critical,
            cache: Arc::new(RwLock::new(HashMap::new())),
            cache_duration: Duration::from_secs(cache_duration_secs),
        }
    }

    pub fn render_vnode(&self) -> String {
        {
            let cache = self.cache.read().expect("Error al leer el cache");
            let cache_key = format!("{:?}{:?}", self.props, self.children);

            if let Some(cached_value) = cache.get(&cache_key) {
                if self.is_cache_valid(cached_value) {
                    return cached_value.value.clone();
                }
            }
        }

        let props = self
            .props
            .iter()
            .map(|(k, v)| format!("{}=\"{}\"", k, v))
            .collect::<Vec<String>>()
            .join(" ");
        let children = self
            .children
            .iter()
            .map(|child| child.render_vnode())
            .collect::<String>();
        let rendered = format!("<{} {}>{}</{}>", self.tag, props, children, self.tag);

        let cache_key = format!("{:?}{:?}", self.props, self.children);
        let mut cache = self.cache.write().expect("Error al escribir en el cache");
        cache.insert(
            cache_key,
            CacheEntry {
                value: rendered.clone(),
                timestamp: Instant::now(),
            },
        );

        rendered
    }

    // Reconciliación por fases
    pub fn reconcile_in_phases(&self, other: &VNode) -> PyResult<()> {
        self.children
            .par_iter()
            .filter(|child| child.is_critical)
            .for_each(|child| {
                if child.tag != other.tag {
                    println!("Reconciliando componente crítico: {}", child.tag);
                }
            });

        self.children
            .iter()
            .filter(|child| !child.is_critical)
            .for_each(|child| {
                if child.tag != other.tag {
                    println!("Reconciliando componente secundario: {}", child.tag);
                }
            });

        Ok(())
    }
}

// Esta función no se expone a Python
impl VNode {
    fn is_cache_valid(&self, entry: &CacheEntry) -> bool {
        entry.timestamp.elapsed() < self.cache_duration
    }
}

// Exponer la función render_vnode a Python
#[pyfunction]
pub fn render_vnode(vnode: &VNode) -> String {
    vnode.render_vnode()
}
