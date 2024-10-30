use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;
use pyo3_stub_gen::derive::*;

#[pyfunction]
#[gen_stub_pyfunction]
fn get_version() -> String {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    VERSION.to_string()
}

#[pymodule]
fn bindings(m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_version, m)?)?;
    Ok(())
}

define_stub_info_gatherer!(stub_info);
