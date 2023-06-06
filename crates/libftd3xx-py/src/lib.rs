use pyo3::prelude::*;

use ::libftd3xx as ftd3xx;
//use ftd3xx::types::Version;

#[pyclass]
#[repr(transparent)]
struct Version {
    pub inner: ftd3xx::Version,
}

#[pymethods]
impl Version {
    #[new]
    fn py_new(value: u32) -> Self {
        Self {
            inner: ftd3xx::Version::with_raw(value),
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
fn library_version() -> PyResult<Version> {
    Ok(Version {
        inner: ftd3xx::library_version().unwrap(),
    })
}

#[pymodule]
fn libftd3xx(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(library_version, m)?)?;
    Ok(())
}
