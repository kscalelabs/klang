use klang::parser::errors::ParseError;
use klang::parser::structs::KlangProgram;
use klang::parser::{parse_file as klang_parse_file, parse_string as klang_parse_string};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3_stub_gen::define_stub_info_gatherer;
use pyo3_stub_gen::derive::*;
use std::path::Path;

#[pyfunction]
#[gen_stub_pyfunction]
fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[pyfunction]
#[gen_stub_pyfunction]
fn parse_file(path: &str) -> PyResult<PyKlangProgram> {
    let program = klang_parse_file(&Path::new(path))
        .map_err(|e| PyValueError::new_err(PyParseError { inner: e }.to_string()))?;
    Ok(PyKlangProgram { inner: program })
}

#[pyfunction]
#[gen_stub_pyfunction]
fn parse_string(input: &str) -> PyResult<PyKlangProgram> {
    let program = klang_parse_string(input)
        .map_err(|e| PyValueError::new_err(PyParseError { inner: e }.to_string()))?;
    Ok(PyKlangProgram { inner: program })
}

#[gen_stub_pyclass]
#[pyclass]
struct PyKlangProgram {
    inner: KlangProgram,
}

impl From<KlangProgram> for PyKlangProgram {
    fn from(program: KlangProgram) -> Self {
        PyKlangProgram { inner: program }
    }
}

#[gen_stub_pymethods]
#[pymethods]
impl PyKlangProgram {
    fn save_binary(&self, path: &str) -> PyResult<()> {
        self.inner
            .save_binary(&Path::new(path))
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn save_text(&self, path: &str) -> PyResult<()> {
        self.inner
            .save_text(&Path::new(path))
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(self.inner.to_string())
    }

    #[staticmethod]
    fn load_binary(path: &str) -> PyResult<PyKlangProgram> {
        let program = KlangProgram::load_binary(&Path::new(path))
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyKlangProgram { inner: program })
    }

    fn to_list(&self) -> PyResult<Vec<Vec<String>>> {
        Ok(self.inner.to_list().clone())
    }
}

#[pyclass]
struct PyParseError {
    inner: ParseError,
}

impl std::fmt::Display for PyParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

#[pymethods]
impl PyParseError {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.inner.to_string())
    }
}

#[pymodule]
fn bindings(m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_version, m)?)?;
    m.add_function(wrap_pyfunction!(parse_file, m)?)?;
    m.add_function(wrap_pyfunction!(parse_string, m)?)?;
    m.add_class::<PyParseError>()?;
    Ok(())
}

define_stub_info_gatherer!(stub_info);
