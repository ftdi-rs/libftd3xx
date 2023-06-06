#![deny(missing_docs)]
///! Contains the safe versions of functions related to libftd3xx-ffi
use libftd3xx_ffi::{prelude::*, FT_DEVICE_LIST_INFO_NODE};
use crate::types::{Error, Result, Version};

/// Get the D3XX user driver library version number.
///
/// Returns [`FT_OK`] if successful, otherwise the return value is an 
/// FT error code. See [`FT_Status`] for more information.
/// 
/// # Example
///
/// ```no_run
/// use libftd3xx::functions::get_library_version;
///
/// let version = get_library_version().unwrap();
/// println!("libftd3xx C library version: {}", version);
/// ```
pub fn get_library_version() -> Result<Version> {
    let mut version: u32 = 0;
    //trace!("FT_GetLibraryVersion(_)");
    let status = unsafe { FT_Status::try_from(FT_GetLibraryVersion(&mut version)) }?;
    if status == FT_OK {
        return Ok(Version::with_raw(version));
    }
    else {
        return Err(Error::APIError(status));
    }
}

/// Get the D3XX kernel driver library version number.
///
/// Returns [`FT_OK`] if successful, otherwise the return value is an 
/// FT error code. See [`FT_Status`] for more information.
/// 
/// # Example
///
/// ```no_run
/// use libftd3xx::functions::get_driver_version;
/// use libftd3xx_ffi::prelude::*;
/// use libftd3xx::types::Error;
///
/// let version = get_driver_version(std::ptr::null_mut());
/// assert_eq!(version, Err(Error::APIError(FT_INVALID_HANDLE)));
/// ```
pub fn get_driver_version(handle: FT_HANDLE) -> Result<Version> {
    let mut version: u32 = 0;
    //trace!("FT_GetLibraryVersion(_)");
    let status = unsafe { FT_Status::try_from(FT_GetDriverVersion(handle, &mut version)) }?;
    if status == FT_OK {
        return Ok(Version::with_raw(version));
    }
    else {
        return Err(Error::APIError(status));
    }
}

/// An application can use this function to get the number of devices attached to the system.
/// It can then allocate space for the device information list and retrieve the list using
/// FT_GetDeviceInfoList or FT_GetDeviceInfoDetail.
/// 
/// If the devices connected to the system change, the device info list will not be updated until
/// FT_CreateDeviceInfoList is called again.
///
/// Returns [`FT_OK`] if successful, otherwise the return value is an 
/// FT error code. See [`FT_Status`] for more information.
/// 
/// # Example
///
/// ```no_run
/// use libftd3xx::functions::create_device_info_list;
///
/// let num_devices = create_device_info_list().unwrap();
/// println!("number of devices: {}", num_devices);
/// ```
pub fn create_device_info_list() -> Result<u32> {
    let mut num_devices: u32 = 0;
    //trace!("FT_CreateDeviceInfoList(_)");
    let status = unsafe { FT_Status::try_from(FT_CreateDeviceInfoList(&mut num_devices)) }?;
    if status == FT_OK {
        return Ok(num_devices);
    }
    else {
        return Err(Error::APIError(status));
    }
}

/// Returns a device information list and the number of D3XX devices in the list.
/// 
/// This function should only be called after calling FT_CreateDeviceInfoList. If the devices
/// connected to the system change, the device info list will not be updated until
/// FT_CreateDeviceInfoList is called again.
/// 
/// Information is not available for devices which are open in other processes. In this case, the
/// Flags parameter of the FT_DEVICE_LIST_INFO_NODE will indicate that the device is open,
/// but other fields will be unpopulated.
/// 
/// The array of FT_DEVICE_LIST_INFO_NODE contains all available data on each device. The
/// storage for the list must be allocated by the application. The number of devices returned
/// by FT_CreateDeviceInfoList can be used to do this.
/// 
/// The Type field of FT_DEVICE_LIST_INFO_NODE structure can be used to determine the
/// device type. Currently, D3XX only supports FT60X devices, FT600 and FT601. The values
/// returned in the Type field are located in the FT_DEVICES enumeration. FT600 and FT601
/// devices have values of FT_DEVICE_600 and FT_DEVICE_601, respectively.
///
/// Returns [`FT_OK`] if successful, otherwise the return value is an 
/// FT error code. See [`FT_Status`] for more information.
/// 
/// # Example
///
/// ```no_run
/// // TODO
/// use libftd3xx::functions::get_device_info_list;
///
/// //let num_devices = get_device_info_list().unwrap();
/// //println!("number of devices: {}", num_devices);
/// ```
pub fn get_device_info_list(devices: &mut Vec<FT_DEVICE_LIST_INFO_NODE>, num_devices: &mut u32) -> Result<()> {
    //trace!("FT_GetDeviceInfoList(_)");
    let status = unsafe { FT_Status::try_from(FT_GetDeviceInfoList(devices.as_mut_ptr() as *mut FT_DEVICE_LIST_INFO_NODE, num_devices)) }?;
    if status == FT_OK {
        return Ok(());
    }
    else {
        return Err(Error::APIError(status));
    }
}

// TODO: FT_GetDeviceInfoDetail


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_library_version() {
        let result = get_library_version();
        //let expected_version: u32 = 0x0;
        cfg_if::cfg_if! {
            if #[cfg(all(target_os = "linux", target_arch = "x86_64"))] {
                // version "1.0.5" is represented as 0x010005
                // let expected_version = 0x01_00_05;
                // for some reason 1.0.5 reports as 0x1000016
                let expected_version = 0x1_00_00_16;
            } else if #[cfg(all(target_os = "linux", target_arch = "x86"))] {
                // version "1.0.5" is represented as 0x010005
                let expected_version = 0x01_00_05;
            } else if #[cfg(all(target_os = "windows", target_arch = "x86_64"))] {
                // version "1.3.0.4" is represented as 0x1030004
                let expected_version = 0x1_03_00_04;
            } else if #[cfg(all(target_os = "windows", target_arch = "x86"))] {
                let expected_version = 0x1_03_00_04;
            } else if #[cfg(all(target_os = "linux", target_arch = "arm"))] {
                // version "1.0.5" is represented as 0x010005
                let expected_version = 0x01_00_05;
            } else if #[cfg(all(target_os = "linux", target_arch = "aarch64"))] {
                // version "1.0.5" is represented as 0x010005
                let expected_version = 0x01_00_05;
            } else if #[cfg(all(target_os = "macos", target_arch = "x86_64"))] {
                // version "1.0.5" is represented as 0x010005
                let expected_version = 0x01_00_05;
            } else if #[cfg(all(target_os = "macos", target_arch = "aarch64"))] {
                // version "1.0.5" is represented as 0x010005
                let expected_version = 0x01_00_05;
            } else {
                std::compile_error!("pre-generated bindings are not avaliable for your target");
            }
        };
        let expected_version = Version::with_raw(expected_version);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), expected_version);
    }

    #[cfg(feature = "hardware_tests")]
    #[test]
    fn test_get_driver_version() {
        use crate::functions::Error::APIError;

        let version = get_driver_version(std::ptr::null_mut());
        assert_eq!(version, Err(APIError(FT_INVALID_HANDLE)));
    }
}
