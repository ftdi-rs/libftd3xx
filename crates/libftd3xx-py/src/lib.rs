use pyo3::prelude::*;
//use libftd3xx_ffi::prelude::*;
use ::libftd3xx as ftd3xx;
use ftd3xx::prelude::*;

pub mod types;

use types::{Error, FtHandle, Result, Version};

#[pyfunction]
fn get_library_version() -> Result<Version> {
    //let library_version = ;
    Ok(Version {
        0: ftd3xx::functions::get_library_version()?,
    })
}

#[pyfunction]
fn get_driver_version(handle: &FtHandle) -> Result<Version> {
    Ok(Version {
        0: ftd3xx::functions::get_driver_version(*handle.0.lock().unwrap())?,
    })
}

#[pyfunction]
fn create_device_info_list() -> Result<u32> {
    Ok(ftd3xx::functions::create_device_info_list()?)
}

/*
#[pyfunction]
fn get_device_info_list(device_count: u32) -> Result<Vec<FtDeviceListInfoNode>> {
    let mut device_count = device_count;
    Ok(ftd3xx::functions::get_device_info_list(&mut device_count)?)
}
*/

#[pymodule]
fn libftd3xx(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_library_version, m)?)?;
    m.add_function(wrap_pyfunction!(get_driver_version, m)?)?;

    m.add_function(wrap_pyfunction!(create_device_info_list, m)?)?;
    //m.add_function(wrap_pyfunction!(get_device_info_list, m)?)?;
    Ok(())
}
