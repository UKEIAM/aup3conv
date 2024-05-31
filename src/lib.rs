use pyo3::prelude::*;

mod tagstack;
mod structure;
mod utils;
pub mod audacity;
pub mod project;

use project::Project;
use crate::structure::Label;


#[pyfunction]
fn open(path: String) -> PyResult<Project> {
    let project = Project::new(&path);
    Ok(project)
}

#[pyfunction]
fn get_labels(path: String) -> PyResult<Option<Vec<Label>>> {
    let project = Project::new(&path);
    Ok(project.labels)
}

#[pymodule]
fn aup3conv(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_labels, m)?)?;
    m.add_function(wrap_pyfunction!(open, m)?)?;
    Ok(())
}
