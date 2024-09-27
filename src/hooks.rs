use dashmap::DashMap;
use once_cell::sync::Lazy;
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use pyo3::wrap_pyfunction;

static GLOBAL_STATE: Lazy<DashMap<String, PyObject>> = Lazy::new(DashMap::new);

#[pyclass]
#[derive(Clone)]
pub struct SetState {
    // Cambiar a `pub`
    key: String,
}

#[pymethods]
impl SetState {
    #[new]
    fn new(key: String) -> Self {
        SetState { key }
    }

    fn set(&self, py: Python, new_value: PyObject) -> PyResult<()> {
        GLOBAL_STATE.insert(self.key.clone(), new_value.into_py(py));
        Ok(())
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Dispatch {
    // Cambiar a `pub`
    key: String,
    reducer: Py<PyAny>,
}

#[pymethods]
impl Dispatch {
    #[new]
    fn new(key: String, reducer: Py<PyAny>) -> Self {
        Dispatch { key, reducer }
    }

    fn dispatch(&self, py: Python, action: PyObject) -> PyResult<()> {
        let reducer = self.reducer.as_ref(py);
        let current_state = GLOBAL_STATE
            .get(&self.key)
            .map(|v| v.clone())
            .unwrap_or_else(|| py.None());

        let args = PyTuple::new(py, &[current_state, action]); // Crear la tupla de argumentos correctamente
        let new_state = reducer.call1(args)?; // Quitar `py` del `call1`

        GLOBAL_STATE.insert(self.key.clone(), new_state.into_py(py));
        Ok(())
    }
}

#[pyfunction]
pub fn use_state(key: &str, initial_value: PyObject) -> PyResult<(PyObject, Py<SetState>)> {
    Python::with_gil(|py| {
        let state = GLOBAL_STATE
            .entry(key.to_string())
            .or_insert_with(|| initial_value.clone());

        let set_state: Py<SetState> = Py::new(py, SetState::new(key.to_string()))?; // Corregir el tipo de retorno
        Ok((state.value().clone(), set_state))
    })
}

#[pyfunction]
pub fn use_lazy_state(key: &str, initial_value: Option<PyObject>) -> PyResult<PyObject> {
    Python::with_gil(|py| {
        let state = GLOBAL_STATE
            .entry(key.to_string())
            .or_insert_with(|| initial_value.clone().unwrap_or_else(|| py.None()));
        Ok(state.value().clone())
    })
}

#[pyfunction]
pub fn use_context(key: &str) -> PyResult<Option<PyObject>> {
    Python::with_gil(|_| Ok(GLOBAL_STATE.get(key).map(|value| value.clone())))
}

#[pyfunction]
pub fn use_reducer(
    key: &str,
    reducer: &PyAny,
    initial_state: PyObject,
) -> PyResult<(PyObject, Py<Dispatch>)> {
    Python::with_gil(|py| {
        let state = GLOBAL_STATE
            .entry(key.to_string())
            .or_insert_with(|| initial_state.clone());

        let reducer_py = reducer.to_object(py).into();
        let dispatch: Py<Dispatch> = Py::new(py, Dispatch::new(key.to_string(), reducer_py))?; // Corregir el tipo de retorno
        Ok((state.value().clone(), dispatch))
    })
}

#[pyfunction]
pub fn use_effect(
    py: Python,
    effect_function: &PyAny,
    dependencies: Option<Vec<PyObject>>,
) -> PyResult<()> {
    let dep = dependencies.unwrap_or_default();
    let args = PyTuple::new(py, dep);
    let effect = effect_function.call1(args)?; // Quitar `py` del `call1`
    if let Ok(cleanup) = effect.getattr("cleanup") {
        cleanup.call0()?; // Quitar `py` de `call0`
    }
    Ok(())
}

#[pymodule]
fn hooks(_py: Python, m: &PyModule) -> PyResult<()> {
    // Cambiar `py` a `_py`
    m.add_class::<SetState>()?;
    m.add_class::<Dispatch>()?;
    m.add_function(wrap_pyfunction!(use_state, m)?)?;
    m.add_function(wrap_pyfunction!(use_lazy_state, m)?)?;
    m.add_function(wrap_pyfunction!(use_context, m)?)?;
    m.add_function(wrap_pyfunction!(use_reducer, m)?)?;
    m.add_function(wrap_pyfunction!(use_effect, m)?)?;
    Ok(())
}
