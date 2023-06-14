use pyo3::exceptions::PyOSError;
use pyo3::prelude::*;
use pyo3::PyErr;
use std::fmt;
use std::sync::{Arc, Mutex};

use ::libftd3xx as ftd3xx;
use ftd3xx::prelude::*;
use ftd3xx::types::Error as FtError;
use libftd3xx_ffi::{prelude::*, FT_DEVICE_LIST_INFO_NODE};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    Generic(FtError),
}
impl std::error::Error for Error {}

impl From<Error> for pyo3::PyErr {
    fn from(err: Error) -> Self {
        PyOSError::new_err(err.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::Generic(FtError::APIError(e)) => write!(f, "API Error occurred: {e:?}"),
            Self::Generic(FtError::CriticalAPIError(s)) => {
                write!(f, "Critical API Error occurred: {s}")
            }
        }
    }
}

impl From<FtError> for Error {
    fn from(value: FtError) -> Self {
        Error::Generic(value)
    }
}

#[pyclass]
#[repr(transparent)]
pub struct Version(pub ftd3xx::types::Version);

#[pymethods]
impl Version {
    #[new]
    fn py_new(value: u32) -> Self {
        Self {
            0: ftd3xx::types::Version::with_raw(value),
        }
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!("<Version {}>", self.0.to_string())
    }
}

macro_rules! define_basic_py_object {
    ($name:ident, $inner_name:ident) => {
        #[pyclass]
        #[derive(Debug)]
        #[repr(transparent)]
        pub struct $name(pub Arc<Mutex<$inner_name>>);

        unsafe impl Send for $name {}
        unsafe impl Sync for $name {}

        #[pymethods]
        impl $name {
            #[new]
            fn py_new() -> Self {
                Self::new()
            }
        }
    };
}

define_basic_py_object!(FtHandle, FT_HANDLE);
//define_basic_py_object!(FtDeviceListInfoNode, Vec<FT_DEVICE_LIST_INFO_NODE>);

impl FtHandle {
    fn new() -> Self {
        Self {
            0: Arc::new(Mutex::new(std::ptr::null_mut())),
        }
    }
}

/*
impl FtDeviceListInfoNode {
    fn new() -> Self {
        Self { 0: Arc::new(Mutex::new(std::ptr::null_mut())) }
    }
}
*/
