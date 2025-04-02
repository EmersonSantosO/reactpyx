use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::collections::HashMap;

/// Virtual DOM node representation
#[pyclass]
#[derive(Debug)]
pub struct VNode {
    #[pyo3(get, set)]
    pub tag: String,
    #[pyo3(get, set)]
    pub props: HashMap<String, PyObject>,
    #[pyo3(get, set)]
    pub children: Vec<Py<VNode>>,
    #[pyo3(get, set)]
    pub is_critical: bool,
    #[pyo3(get, set)]
    pub cache_duration_secs: u64,
    #[pyo3(get, set)]
    pub key: Option<String>,
}

#[pymethods]
impl VNode {
    #[new]
    #[pyo3(signature = (tag, props, children, is_critical, cache_duration_secs, key=None))]
    fn new(
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
            cache_duration_secs,
            key,
        }
    }

    /// Renders the virtual node as an HTML string
    pub fn render(&self, py: Python) -> PyResult<String> {
        let mut html = format!("<{}", self.tag);

        // Add attributes
        for (key, value) in &self.props {
            let value_str: String = value.extract(py)?;
            html.push_str(&format!(" {}=\"{}\"", key, value_str));
        }
        html.push('>');

        // Add children
        for child in &self.children {
            let child_node = child.borrow(py);
            let child_html = child_node.render(py)?;

            html.push_str(&child_html);
        }

        html.push_str(&format!("</{}>", self.tag));

        Ok(html)
    }
    
    /// Creates a deep clone of this node
    pub fn clone_node(&self, py: Python) -> PyResult<Py<VNode>> {
        let cloned_children = self.children.iter()
            .map(|child| child.borrow(py).clone_node(py))
            .collect::<Result<Vec<_>, _>>()?;
        
        let cloned_props = self.props.clone();

        let new_node = VNode {
            tag: self.tag.clone(),
            props: cloned_props,
            children: cloned_children,
            is_critical: self.is_critical,
            cache_duration_secs: self.cache_duration_secs,
            key: self.key.clone(),
        };
        
        Py::new(py, new_node)
    }
    
    /// Adds a new child to this node
    pub fn add_child(&mut self, py: Python, child: Py<VNode>) -> PyResult<()> {
        self.children.push(child);
        Ok(())
    }
    
    /// Adds a new prop to this node
    pub fn add_prop(&mut self, py: Python, key: &str, value: PyObject) -> PyResult<()> {
        self.props.insert(key.to_string(), value);
        Ok(())
    }
}

/// Types of patch operations for virtual nodes
#[pyclass]
#[derive(Debug)]
pub enum Patch {
    AddProp { key: String, value: PyObject },
    RemoveProp { key: String },
    UpdateProp { key: String, value: PyObject },
    AddChild { child: Py<VNode> },
    RemoveChild { index: usize },
    ReplaceChild { index: usize, child: Py<VNode> },
}

#[pymethods]
impl Patch {
    /// Applies a patch to the virtual node
    pub fn apply(&self, node: &mut VNode, py: Python) -> PyResult<()> {
        match self {
            Patch::AddProp { key, value } => {
                node.props.insert(key.clone(), value.clone_ref(py));
            }
            Patch::RemoveProp { key } => {
                node.props.remove(key);
            }
            Patch::UpdateProp { key, value } => {
                node.props.insert(key.clone(), value.clone_ref(py));
            }
            Patch::AddChild { child } => {
                node.children.push(child.clone_ref(py));
            }
            Patch::RemoveChild { index } => {
                node.children.remove(*index);
            }
            Patch::ReplaceChild { index, child } => {
                node.children[*index] = child.clone_ref(py);
            }
        }
        Ok(())
    }
}

/// Calculates differences between two virtual nodes
#[pyfunction]
pub fn diff_nodes(py: Python, old_node: &VNode, new_node: &VNode) -> Vec<Patch> {
    let mut patches = Vec::new();

    // Compare properties
    for (key, new_value) in &new_node.props {
        match old_node.props.get(key) {
            Some(old_value) if old_value.compare(new_value).unwrap_or(false) != std::cmp::Ordering::Equal => {
                patches.push(Patch::UpdateProp {
                    key: key.clone(),
                    value: new_value.clone_ref(py),
                });
            }
            None => {
                patches.push(Patch::AddProp {
                    key: key.clone(),
                    value: new_value.clone_ref(py),
                });
            }
            _ => {}
        }
    }

    // Detect removed properties
    for key in old_node.props.keys() {
        if !new_node.props.contains_key(key) {
            patches.push(Patch::RemoveProp { key: key.clone() });
        }
    }

    // Compare children (simplified approach)
    let min_children_len = old_node.children.len().min(new_node.children.len());
    for index in 0..min_children_len {
        let old_child = old_node.children[index].borrow(py);
        let new_child = new_node.children[index].borrow(py);

        if old_child.tag != new_child.tag {
            patches.push(Patch::ReplaceChild {
                index,
                child: new_node.children[index].clone_ref(py),
            });
        }
    }

    // Add new children
    if new_node.children.len() > old_node.children.len() {
        for index in old_node.children.len()..new_node.children.len() {
            patches.push(Patch::AddChild {
                child: new_node.children[index].clone_ref(py),
            });
        }
    }

    // Remove old children
    if old_node.children.len() > new_node.children.len() {
        for index in (new_node.children.len()..old_node.children.len()).rev() {
            patches.push(Patch::RemoveChild { index });
        }
    }

    patches
}

#[pymodule]
fn virtual_dom(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<VNode>()?;
    m.add_class::<Patch>()?;
    m.add_function(wrap_pyfunction!(diff_nodes, m)?)?;
    Ok(())
}
