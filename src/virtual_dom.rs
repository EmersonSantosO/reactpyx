use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::collections::HashMap;

/// Representación de un nodo virtual en el DOM
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

    /// Renderiza el nodo virtual como una cadena HTML
    pub fn render(&self, py: Python) -> PyResult<String> {
        let mut html = format!("<{}", self.tag);

        // Añade atributos
        for (key, value) in &self.props {
            let value_str: String = value.extract(py)?;
            html.push_str(&format!(" {}=\"{}\"", key, value_str));
        }
        html.push('>');

        // Añade hijos
        for child in &self.children {
            let child_node = child.borrow(py);
            let child_html = child_node.render(py)?;

            html.push_str(&child_html);
        }

        html.push_str(&format!("</{}>", self.tag));

        Ok(html)
    }
}

/// Tipos de modificaciones (parches) para nodos virtuales.
#[derive(Debug)]
pub enum Patch {
    AddProp { key: String, value: PyObject },
    RemoveProp { key: String },
    UpdateProp { key: String, value: PyObject },
    AddChild { child: Py<VNode> },
    RemoveChild { index: usize },
    ReplaceChild { index: usize, child: Py<VNode> },
}

impl Patch {
    /// Aplica un conjunto de "parches" al nodo virtual.
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

/// Calcula las diferencias entre dos nodos virtuales.
pub fn diff_nodes(py: Python, old_node: &VNode, new_node: &VNode) -> Vec<Patch> {
    let mut patches = Vec::new();

    // Compara propiedades
    for (key, new_value) in &new_node.props {
        match old_node.props.get(key) {
            Some(old_value) if !old_value.compare(new_value).unwrap_or(false) => {
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

    // Detecta propiedades eliminadas
    for key in old_node.props.keys() {
        if !new_node.props.contains_key(key) {
            patches.push(Patch::RemoveProp { key: key.clone() });
        }
    }

    // Compara hijos (simplificación)
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

    // Añade nuevos hijos
    if new_node.children.len() > old_node.children.len() {
        for index in old_node.children.len()..new_node.children.len() {
            patches.push(Patch::AddChild {
                child: new_node.children[index].clone_ref(py),
            });
        }
    }

    // Elimina hijos antiguos
    if old_node.children.len() > new_node.children.len() {
        for index in (new_node.children.len()..old_node.children.len()).rev() {
            patches.push(Patch::RemoveChild { index });
        }
    }

    patches
}

#[pymodule]
fn virtual_dom(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyclass!(VNode))?;
    Ok(())
}
