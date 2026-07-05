use pyo3::prelude::*;

#[pyclass]
pub struct A {}

#[pymethods]
impl A {
    #[new]
    pub fn new() -> Self {
        Self {}
    }
}
