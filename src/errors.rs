use colored::Colorize;
use log::{error, warn};
use pyo3::prelude::*;

/// Handles general errors
pub fn handle_error(err: &str) {
    error!("{}", err.red());
    println!("{}", err.red());
}

/// Handles general warnings
pub fn handle_warning(warning: &str) {
    warn!("{}", warning.yellow());
    println!("{}", warning.yellow());
}

#[pymodule]
fn errors(m: &PyModule) -> PyResult<()> {
    #[pyfunction]
    pub fn raise_error(err_msg: &str) -> PyResult<()> {
        handle_error(err_msg);
        Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(err_msg))
    }

    m.add_function(wrap_pyfunction_bound!(raise_error, m)?)?;
    Ok(())
}
