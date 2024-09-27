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

// Define las posibles operaciones de parcheo
#[derive(Debug, Clone)]
pub enum Patch {
    Replace(VNode),
    AddProp(String, PyObject),
    RemoveProp(String),
    UpdateProp(String, PyObject),
    AppendChildren(Vec<VNode>),
    RemoveChildren(usize, usize), // Rango de índices de hijos a eliminar
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

    #[getter]
    pub fn key(&self) -> Option<String> {
        self.key.clone()
    }

    // Función para calcular la diferencia entre dos VNodes
    pub fn diff(&self, new_vnode: &VNode, py: Python) -> PyResult<Vec<Patch>> {
        let mut patches = Vec::new();

        // 1. Comparar tags
        if self.tag != new_vnode.tag {
            patches.push(Patch::Replace(new_vnode.clone()));
            return Ok(patches); // Si los tags son diferentes, se reemplaza todo el nodo
        }

        // 2. Comparar props
        for (key, value) in &self.props {
            if let Some(new_value) = new_vnode.props.get(key) {
                if value != new_value {
                    patches.push(Patch::UpdateProp(key.clone(), new_value.clone()));
                }
            } else {
                patches.push(Patch::RemoveProp(key.clone()));
            }
        }
        for (key, value) in &new_vnode.props {
            if !self.props.contains_key(key) {
                patches.push(Patch::AddProp(key.clone(), value.clone()));
            }
        }

        // 3. Comparar hijos (usando claves si están disponibles)
        let mut old_children_map: HashMap<Option<String>, &Py<VNode>> = HashMap::new();
        for child in &self.children {
            old_children_map.insert(child.borrow(py).key(), child);
        }

        let mut i = 0;
        let mut new_children = Vec::new();
        for new_child in &new_vnode.children {
            let new_child_borrowed = new_child.borrow(py);
            if let Some(old_child) = old_children_map.get(&new_child_borrowed.key()) {
                // El hijo ya existe, calcular la diferencia recursivamente
                patches.extend(old_child.borrow(py).diff(&new_child_borrowed, py)?);
                new_children.push(old_child.clone());
            } else {
                // El hijo es nuevo, agregarlo
                patches.push(Patch::AppendChildren(vec![new_child_borrowed.clone()]));
                new_children.push(new_child.clone());
            }
            i += 1;
        }

        // Eliminar hijos que ya no están presentes
        if self.children.len() > new_children.len() {
            let start = new_children.len();
            let end = self.children.len();
            patches.push(Patch::RemoveChildren(start, end));
        }

        Ok(patches)
    }
}
