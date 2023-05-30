use pyo3::prelude::*;

use ::libftd3xx as ftd3xx;

#[pyfunction]
fn library_version() -> PyResult<u32> {
    return Ok(ftd3xx::library_version().unwrap());
}

#[pymodule]
fn libftd3xx(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(library_version, m)?)?;
    Ok(())
}