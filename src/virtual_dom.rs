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
    pub props: HashMap<String, Py<PyAny>>,
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
        props: HashMap<String, Py<PyAny>>,
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
            // Handle event handlers for SSR
            if key.starts_with("on") {
                let event_name = key[2..].to_lowercase();

                // Register handler in Python registry
                let registry = py.import("reactpyx.registry")?;
                let handler_id: String = registry
                    .call_method1("register_handler", (value,))?
                    .extract()?;

                html.push_str(&format!(" data-on-{}=\"{}\"", event_name, handler_id));
                continue;
            }

            // Safely convert value to string
            let value_str: String = if let Ok(s) = value.extract::<String>(py) {
                s
            } else {
                // If not a string, try to convert using str()
                value.bind(py).str()?.to_string()
            };

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

    /// Serializes the node to a Python dictionary (for JSON serialization)
    pub fn to_dict(&self, py: Python) -> PyResult<Py<PyAny>> {
        let dict = pyo3::types::PyDict::new(py);
        dict.set_item("tag", &self.tag)?;

        let props_dict = pyo3::types::PyDict::new(py);
        for (key, value) in &self.props {
            // For props, we need to be careful about what we send to the client
            // Event handlers are just names/flags here
            if key.starts_with("on") {
                props_dict.set_item(key, true)?;
            } else {
                // Try to keep basic types, fallback to string
                if let Ok(s) = value.extract::<String>(py) {
                    props_dict.set_item(key, s)?;
                } else if let Ok(i) = value.extract::<i64>(py) {
                    props_dict.set_item(key, i)?;
                } else if let Ok(b) = value.extract::<bool>(py) {
                    props_dict.set_item(key, b)?;
                } else {
                    let s = value.bind(py).str()?.to_string();
                    props_dict.set_item(key, s)?;
                }
            }
        }
        dict.set_item("props", props_dict)?;

        let children_list = pyo3::types::PyList::empty(py);
        for child in &self.children {
            let child_node = child.borrow(py);
            children_list.append(child_node.to_dict(py)?)?;
        }
        dict.set_item("children", children_list)?;

        if let Some(key) = &self.key {
            dict.set_item("key", key)?;
        }

        Ok(dict.into())
    }

    /// Creates a deep clone of this node
    pub fn clone_node(&self, py: Python) -> PyResult<Py<VNode>> {
        let cloned_children = self
            .children
            .iter()
            .map(|child| child.borrow(py).clone_node(py))
            .collect::<Result<Vec<_>, _>>()?;

        // Manual clone of props because Py<PyAny> might not implement Clone in this context
        // or to be explicit about using the GIL token
        let mut cloned_props = HashMap::new();
        for (k, v) in &self.props {
            cloned_props.insert(k.clone(), v.clone_ref(py));
        }

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
    pub fn add_child(&mut self, _py: Python, child: Py<VNode>) -> PyResult<()> {
        self.children.push(child);
        Ok(())
    }

    /// Adds a new prop to this node
    pub fn add_prop(&mut self, _py: Python, key: &str, value: Py<PyAny>) -> PyResult<()> {
        self.props.insert(key.to_string(), value);
        Ok(())
    }
}

/// Types of patch operations for virtual nodes
#[pyclass]
#[derive(Debug)]
pub enum Patch {
    AddProp { key: String, value: Py<PyAny> },
    RemoveProp { key: String },
    UpdateProp { key: String, value: Py<PyAny> },
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
            Some(old_value) => {
                // Use bind(py) to get Bound<'_, PyAny> which has compare
                let old_bound = old_value.bind(py);
                let new_bound = new_value.bind(py);

                if old_bound
                    .compare(new_bound)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    != std::cmp::Ordering::Equal
                {
                    patches.push(Patch::UpdateProp {
                        key: key.clone(),
                        value: new_value.clone_ref(py),
                    });
                }
            }
            None => {
                patches.push(Patch::AddProp {
                    key: key.clone(),
                    value: new_value.clone_ref(py),
                });
            }
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

/// Adds the Virtual DOM module to the Python module
#[pymodule]
fn virtual_dom(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<VNode>()?;
    m.add_class::<Patch>()?;
    m.add_function(wrap_pyfunction!(diff_nodes, m)?)?;
    Ok(())
}
