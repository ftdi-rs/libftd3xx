use ::libftd3xx as ftd3xx;
use pyo3::prelude::*;

pub mod types;

use types::{Ft60xConfiguration, FtDeviceListInfoNode, FtException, FtHandle, Result, Version};

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
    Ok(device_info_list
        .into_iter()
        .map(|x| FtDeviceListInfoNode::from(x))
        .collect())
}

#[pyfunction]
fn create_by_index(index: u32) -> Result<FtHandle> {
    Ok(FtHandle::from(ftd3xx::functions::create_by_index(index)?))
}

#[pyfunction]
fn create_by_description(description: String) -> Result<FtHandle> {
    Ok(FtHandle::from(ftd3xx::functions::create_by_description(
        description,
    )?))
}

#[pyfunction]
fn create_by_serial_number(serial_number: String) -> Result<FtHandle> {
    Ok(FtHandle::from(ftd3xx::functions::create_by_serial_number(
        serial_number,
    )?))
}

#[pyfunction]
fn close(handle: &FtHandle) -> Result<()> {
    Ok(ftd3xx::functions::close(*handle.0.lock().unwrap())?)
}

#[pyfunction]
fn get_chip_configuration(handle: &FtHandle) -> Result<Ft60xConfiguration> {
    Ok(Ft60xConfiguration::from(
        ftd3xx::functions::get_chip_configuration(*handle.0.lock().unwrap())?,
    ))
}

#[pyfunction]
fn set_chip_configuration(handle: &FtHandle, config: Option<&Ft60xConfiguration>) -> Result<()> {
    let result = match &config {
        Some(c) => ftd3xx::functions::set_chip_configuration(
            *handle.0.lock().unwrap(),
            Some(*c.0.lock().unwrap()),
        )?,
        None => ftd3xx::functions::set_chip_configuration(*handle.0.lock().unwrap(), None)?,
    };
    Ok(result)
}

#[pymodule]
fn libftd3xx(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("FtException", py.get_type::<FtException>())?;
    m.add("Ft60xConfiguration", py.get_type::<Ft60xConfiguration>())?;

    m.add_function(wrap_pyfunction!(get_library_version, m)?)?;
    m.add_function(wrap_pyfunction!(get_driver_version, m)?)?;

    m.add_function(wrap_pyfunction!(create_device_info_list, m)?)?;
    m.add_function(wrap_pyfunction!(get_device_info_list, m)?)?;

    m.add_function(wrap_pyfunction!(create_by_index, m)?)?;
    m.add_function(wrap_pyfunction!(create_by_description, m)?)?;
    m.add_function(wrap_pyfunction!(create_by_serial_number, m)?)?;
    m.add_function(wrap_pyfunction!(close, m)?)?;

    m.add_function(wrap_pyfunction!(get_chip_configuration, m)?)?;
    m.add_function(wrap_pyfunction!(set_chip_configuration, m)?)?;

    Ok(())
}
