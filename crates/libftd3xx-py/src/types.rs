use libftd3xx_ffi::DWORD;
use libftd3xx_ffi::ULONG;
use pyo3::create_exception;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use std::ffi::CStr;
use std::fmt;
use std::sync::{Arc, Mutex};

use ::libftd3xx as ftd3xx;
use ftd3xx::types::Error as FtError;
use libftd3xx_ffi::{prelude::*, FT_DEVICE_LIST_INFO_NODE};

pub type Result<T> = std::result::Result<T, Error>;

create_exception!(libftd3xx, FtException, PyRuntimeError);

#[derive(Debug, PartialEq)]
pub enum Error {
    Generic(FtError),
}
impl std::error::Error for Error {}

impl From<Error> for pyo3::PyErr {
    fn from(err: Error) -> Self {
        let args = match &err {
            Error::Generic(FtError::APIError(e)) => (err.to_string(), *e as i64),
            Error::Generic(FtError::CriticalAPIError(_)) => (err.to_string(), -1 as i64),
        };
        FtException::new_err(args)
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

    #[getter]
    fn major(&self) -> PyResult<u8> {
        Ok(self.0.major)
    }

    #[getter]
    fn minor(&self) -> PyResult<u8> {
        Ok(self.0.minor)
    }

    #[getter]
    fn build(&self) -> PyResult<u16> {
        Ok(self.0.build)
    }
}

macro_rules! define_basic_py_object {
    ($name:ident, $inner_name:ty) => {
        #[pyclass]
        #[derive(Debug)]
        #[repr(transparent)]
        pub struct $name(pub Arc<Mutex<$inner_name>>);

        // Arc is only Send if T is Send so lets mark it as safe here
        unsafe impl Send for $name {}

        #[pymethods]
        impl $name {
            #[new]
            fn py_new() -> Self {
                Self::new()
            }
        }
    };
}

macro_rules! define_basic_py_object_no_new {
    ($name:ident, $inner_name:ty) => {
        #[pyclass]
        #[derive(Debug)]
        #[repr(transparent)]
        pub struct $name(pub Arc<Mutex<$inner_name>>);

        // Arc is only Send if T is Send so lets mark it as safe here
        unsafe impl Send for $name {}
    };
}

define_basic_py_object!(FtHandle, FT_HANDLE);

impl FtHandle {
    fn new() -> Self {
        Self {
            0: Arc::new(Mutex::new(std::ptr::null_mut())),
        }
    }

    pub fn from(handle: FT_HANDLE) -> Self {
        Self {
            0: Arc::new(Mutex::new(handle)),
        }
    }
}

define_basic_py_object_no_new!(FtDeviceListInfoNode, FT_DEVICE_LIST_INFO_NODE);

impl FtDeviceListInfoNode {
    fn new() -> Self {
        Self {
            0: Arc::new(Mutex::new(FT_DEVICE_LIST_INFO_NODE::default())),
        }
    }

    pub fn from(data: FT_DEVICE_LIST_INFO_NODE) -> Self {
        Self {
            0: Arc::new(Mutex::new(data)),
        }
    }
}

#[pymethods]
impl FtDeviceListInfoNode {
    #[new]
    fn py_new() -> Self {
        Self::new()
    }

    #[getter(Flags)]
    fn flags(&self) -> PyResult<ULONG> {
        let info = *self.0.lock().unwrap();
        Ok(info.Flags)
    }

    #[getter(Type)]
    fn get_type(&self) -> PyResult<ULONG> {
        let info = *self.0.lock().unwrap();
        Ok(info.Type)
    }

    #[getter(ID)]
    fn id(&self) -> PyResult<ULONG> {
        let info = *self.0.lock().unwrap();
        Ok(info.ID)
    }

    #[getter(LocID)]
    fn loc_id(&self) -> PyResult<DWORD> {
        let info = *self.0.lock().unwrap();
        Ok(info.LocId)
    }

    #[getter(SerialNumber)]
    fn serial_number(&self) -> PyResult<String> {
        let info = *self.0.lock().unwrap();
        let cstr_sn = unsafe { CStr::from_ptr(info.SerialNumber.as_ptr()) };
        let sn = String::from_utf8_lossy(cstr_sn.to_bytes()).to_string();
        Ok(sn)
    }

    #[getter(Description)]
    fn description(&self) -> PyResult<String> {
        let info = *self.0.lock().unwrap();
        let cstr_sn = unsafe { CStr::from_ptr(info.Description.as_ptr()) };
        let description = String::from_utf8_lossy(cstr_sn.to_bytes()).to_string();
        Ok(description)
    }

    #[getter(ftHandle)]
    fn ft_handle(&self) -> PyResult<FtHandle> {
        let info = *self.0.lock().unwrap();
        Ok(FtHandle::from(info.ftHandle))
    }
}
