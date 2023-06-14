use pyo3::prelude::*;
use ::libftd3xx as ftd3xx;

pub mod types;

use types::{FtHandle, FtDeviceListInfoNode, Result, Version};

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


#[pyfunction]
fn get_device_info_list(device_count: u32) -> Result<Vec<FtDeviceListInfoNode>> {
    let mut device_count = device_count;
    let device_info_list = ftd3xx::functions::get_device_info_list(&mut device_count)?;
    Ok(
        device_info_list.into_iter().map(|x| FtDeviceListInfoNode::from(x)).collect(),
    )
}

#[pyfunction]
fn create_by_index(index: u32) -> Result<FtHandle> {
    Ok(FtHandle::from(ftd3xx::functions::create_by_index(index)?))
}

#[pyfunction]
fn create_by_description(description: String) -> Result<FtHandle> {
    Ok(FtHandle::from(ftd3xx::functions::create_by_description(description)?))
}

#[pyfunction]
fn create_by_serial_number(serial_number: String) -> Result<FtHandle> {
    Ok(FtHandle::from(ftd3xx::functions::create_by_description(serial_number)?))
}

#[pyfunction]
fn close(handle: &FtHandle) -> Result<()> {
    Ok(ftd3xx::functions::close(*handle.0.lock().unwrap())?)
}

#[pymodule]
fn libftd3xx(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_library_version, m)?)?;
    m.add_function(wrap_pyfunction!(get_driver_version, m)?)?;

    m.add_function(wrap_pyfunction!(create_device_info_list, m)?)?;
    m.add_function(wrap_pyfunction!(get_device_info_list, m)?)?;

    m.add_function(wrap_pyfunction!(create_by_index, m)?)?;
    m.add_function(wrap_pyfunction!(create_by_description, m)?)?;
    m.add_function(wrap_pyfunction!(create_by_serial_number, m)?)?;
    m.add_function(wrap_pyfunction!(close, m)?)?;
    Ok(())
}
