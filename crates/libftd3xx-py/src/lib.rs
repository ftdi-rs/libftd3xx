use pyo3::prelude::*;

use ::libftd3xx as ftd3xx;

#[pyclass]
#[repr(transparent)]
struct Version {
    pub inner: ftd3xx::types::Version,
}

#[pymethods]
impl Version {
    #[new]
    fn py_new(value: u32) -> Self {
        Self {
            inner: ftd3xx::types::Version::with_raw(value),
        }
    }

    fn __str__(&self) -> String {
        self.inner.to_string()
    }

    fn __repr__(&self) -> String {
        format!("<Version {}>", self.inner.to_string())
    }
}

#[pyfunction]
fn get_library_version() -> PyResult<Version> {
    Ok(Version {
        inner: ftd3xx::functions::get_library_version().unwrap(),
    })
}

#[pymodule]
fn libftd3xx(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_library_version, m)?)?;
    Ok(())
}
