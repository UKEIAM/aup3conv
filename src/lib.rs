use pyo3::prelude::*;
use pyo3::exceptions::PyIOError;

mod tagstack;
mod structure;
mod io;
pub mod utils;
pub mod audacity;
pub mod project;

use project::Project;


#[pyfunction]
fn open(path: String) -> PyResult<Project> {
    match Project::open(&path) {
        Ok(project) => Ok(project),
        Err(_) => Err(PyIOError::new_err(format!("File not fould '{}'.", &path)))
    }
}


#[pymodule]
fn _aup3conv(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(open, m)?)?;
    Ok(())
}
