use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;
use pyo3_stub_gen::derive::*;

#[pyfunction]
#[gen_stub_pyfunction]
fn add(a: u64, b: u64) -> u64 {
    a + b
}

#[pymodule]
fn bindings(m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    Ok(())
}

define_stub_info_gatherer!(stub_info);
