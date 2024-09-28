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
    cache: Arc<Mutex<HashMap<String, CacheEntry>>>,
    cache_duration: Duration,
    key: Option<String>,
}

#[pyclass]
#[derive(Debug, Clone)]
pub enum Patch {
    Replace { vnode: VNode },
    AddProp { key: String, value: PyObject },
    RemoveProp { key: String },
    UpdateProp { key: String, value: PyObject },
    AppendChildren { children: Vec<VNode> },
    RemoveChildren { start: usize, end: usize },
}

#[pymethods]
impl VNode {
    #[new]
    pub fn new(
        tag: String,
        props: HashMap<String, PyObject>,
        children: Vec<Py<VNode>>,
        is_critical: bool,
        cache_duration_secs: u64,
        key: Option<String>,
    ) -> Self {
        VNode {
            tag,
            props,
            children,
            is_critical,
            cache: Arc::new(Mutex::new(HashMap::new())),
            cache_duration: Duration::from_secs(cache_duration_secs),
            key,
        }
    }

    pub fn clean_cache(&self) {
        if let Ok(mut cache) = self.cache.lock() {
            cache.retain(|_, entry| entry.timestamp.elapsed() < self.cache_duration);
        }
    }

    pub fn set_cache_duration(&mut self, duration_secs: u64) {
        self.cache_duration = Duration::from_secs(duration_secs);
    }

    pub fn render_vnode(&self, py: Python) -> PyResult<String> {
        let cache_key = format!("{:?}{:?}", self.tag, self.props);
        self.clean_cache();

        if !self.is_critical {
            if let Ok(cache) = self.cache.lock() {
                if let Some(entry) = cache.get(&cache_key) {
                    if entry.timestamp.elapsed() < self.cache_duration {
                        return Ok(entry.value.clone());
                    }
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
                let child_vnode = child.borrow(py);
                child_vnode.render_vnode(py)
            })
            .collect::<PyResult<String>>()?;

        let rendered = format!("<{}{}>{}</{}>", self.tag, props_str, children_str, self.tag);

        if !self.is_critical {
            if let Ok(mut cache) = self.cache.lock() {
                cache.insert(
                    cache_key,
                    CacheEntry {
                        value: rendered.clone(),
                        timestamp: Instant::now(),
                    },
                );
            }
        }

        Ok(rendered)
    }

    #[getter]
    pub fn key(&self) -> Option<String> {
        self.key.clone()
    }

    // Función para calcular la diferencia entre dos VNodes
    pub fn diff(&self, new_vnode: &VNode, py: Python) -> PyResult<Vec<Patch>> {
        let mut patches = Vec::new();

        // 1. Comparar tags
        if self.tag != new_vnode.tag {
            patches.push(Patch::Replace {
                vnode: new_vnode.clone(),
            });
            return Ok(patches); // Si los tags son diferentes, se reemplaza todo el nodo
        }

        // 2. Comparar props
        for (key, value) in &self.props {
            if let Some(new_value) = new_vnode.props.get(key) {
                if !value
                    .bind(py)
                    .call_method1("__eq__", (new_value,))?
                    .extract::<bool>()?
                {
                    patches.push(Patch::UpdateProp {
                        key: key.clone(),
                        value: new_value.clone(),
                    });
                }
            } else {
                patches.push(Patch::RemoveProp { key: key.clone() });
            }
        }
        for (key, value) in &new_vnode.props {
            if !self.props.contains_key(key) {
                patches.push(Patch::AddProp {
                    key: key.clone(),
                    value: value.clone(),
                });
            }
        }

        // 3. Comparar hijos (usando claves si están disponibles)
        let mut old_children_map: HashMap<Option<String>, &Py<VNode>> = HashMap::new();
        for child in &self.children {
            old_children_map.insert(child.borrow(py).key(), child);
        }

        let mut new_children = Vec::new();
        for new_child in &new_vnode.children {
            let new_child_borrowed = new_child.borrow(py);
            if let Some(old_child) = old_children_map.get(&new_child_borrowed.key()) {
                // El hijo ya existe, calcular la diferencia recursivamente
                patches.extend(old_child.borrow(py).diff(&new_child_borrowed, py)?);
                new_children.push(*old_child); // Inserta una referencia
            } else {
                // El hijo es nuevo, agregarlo
                patches.push(Patch::AppendChildren {
                    children: vec![new_child_borrowed.clone()],
                });
                new_children.push(new_child); // Inserta una referencia
            }
        }

        // Eliminar hijos que ya no están presentes
        if self.children.len() > new_children.len() {
            let start = new_children.len();
            let end = self.children.len();
            patches.push(Patch::RemoveChildren { start, end });
        }

        Ok(patches)
    }
}
