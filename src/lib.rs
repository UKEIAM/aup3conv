use pyo3::prelude::*;

mod tagstack;
mod structure;
mod io;
pub mod utils;
pub mod audacity;
pub mod project;

use project::Project;


#[pyfunction]
fn open(path: String) -> PyResult<Project> {
    let project = Project::open(&path);
    Ok(project)
}


#[pymodule]
fn _aup3conv(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(open, m)?)?;
    Ok(())
}
