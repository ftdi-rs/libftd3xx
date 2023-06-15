#![deny(missing_docs)]
use std::ffi::c_void;

use crate::types::{Error, Result, Version};
///! Contains the safe versions of functions related to libftd3xx-ffi
use libftd3xx_ffi::{prelude::*, FT_60XCONFIGURATION, FT_DEVICE_LIST_INFO_NODE, FT_HANDLE};

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
    } else {
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
    } else {
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
    } else {
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
pub fn get_device_info_list(num_devices: &mut u32) -> Result<Vec<FT_DEVICE_LIST_INFO_NODE>> {
    //trace!("FT_GetDeviceInfoList(_)");
    let mut info_list: Vec<FT_DEVICE_LIST_INFO_NODE> = Vec::<FT_DEVICE_LIST_INFO_NODE>::new();
    info_list.resize(*num_devices as usize, FT_DEVICE_LIST_INFO_NODE::default());
    let status = unsafe {
        FT_Status::try_from(FT_GetDeviceInfoList(
            info_list.as_mut_ptr() as *mut FT_DEVICE_LIST_INFO_NODE,
            num_devices,
        ))
    }?;
    if status == FT_OK {
        return Ok(info_list);
    } else {
        return Err(Error::APIError(status));
    }
}

// TODO: FT_GetDeviceInfoDetail

/// Open the device and return a handle which will be used for subsequent accesses.
///
/// Using [`FT_OPEN_BY_SERIAL_NUMBER`] allows an application to open a device that has the
/// specified Serial Number. Using [`FT_OPEN_BY_DESCRIPTION`] allows an application to open a
/// device that has the specified Product Description. Using [`FT_OPEN_BY_INDEX`] is a fall-back
/// option for instances where the devices connected to a machine do not have a unique Serial
/// Number or Product Description.
///
/// Returns [`FT_OK`] if successful, otherwise the return value is an
/// FT error code. See [`FT_Status`] for more information.
///
/// [`FT_OPEN_BY_SERIAL_NUMBER`]: libftd3xx_ffi::FT_OPEN_BY_SERIAL_NUMBER
/// [`FT_OPEN_BY_DESCRIPTION`]: libftd3xx_ffi::FT_OPEN_BY_DESCRIPTION
/// [`FT_OPEN_BY_INDEX`]: libftd3xx_ffi::FT_OPEN_BY_INDEX
/// # Example
///
/// ```no_run
/// use libftd3xx::functions::{create_by_index};
///
/// let handle = create_by_index(0).unwrap();
/// ```
pub fn create_by_index(index: libftd3xx_ffi::ULONG) -> Result<FT_HANDLE> {
    //trace!("FT_Create(_)");
    let mut handle: FT_HANDLE = std::ptr::null_mut();
    let pv_arg = index as *mut std::ffi::c_ulong as *mut std::ffi::c_void;
    let status = unsafe { FT_Status::try_from(FT_Create(pv_arg, FT_OPEN_BY_INDEX, &mut handle)) }?;
    if status == FT_OK {
        return Ok(handle);
    } else {
        return Err(Error::APIError(status));
    }
}

/// Open the device and return a handle which will be used for subsequent accesses.
///
/// Using [`FT_OPEN_BY_SERIAL_NUMBER`] allows an application to open a device that has the
/// specified Serial Number. Using [`FT_OPEN_BY_DESCRIPTION`] allows an application to open a
/// device that has the specified Product Description. Using [`FT_OPEN_BY_INDEX`] is a fall-back
/// option for instances where the devices connected to a machine do not have a unique Serial
/// Number or Product Description.
///
/// Returns [`FT_OK`] if successful, otherwise the return value is an
/// FT error code. See [`FT_Status`] for more information.
///
/// [`FT_OPEN_BY_SERIAL_NUMBER`]: libftd3xx_ffi::FT_OPEN_BY_SERIAL_NUMBER
/// [`FT_OPEN_BY_DESCRIPTION`]: libftd3xx_ffi::FT_OPEN_BY_DESCRIPTION
/// [`FT_OPEN_BY_INDEX`]: libftd3xx_ffi::FT_OPEN_BY_INDEX
///
/// # Example
///
/// ```no_run
/// use libftd3xx::functions::{create_by_serial_number};
///
/// let handle = create_by_serial_number("MySerialNumber").unwrap();
/// ```
pub fn create_by_serial_number<S: Into<String>>(serial: S) -> Result<FT_HANDLE> {
    //trace!("FT_Create(_)");
    let mut handle: FT_HANDLE = std::ptr::null_mut();
    let mut buffer: Vec<u8> = Vec::from(serial.into());
    // Make sure our string is null terminated
    if !buffer.ends_with(&[0u8]) {
        buffer.push(0u8);
    }

    let status = unsafe {
        FT_Status::try_from(FT_Create(
            buffer.as_mut_ptr() as *mut std::ffi::c_void,
            FT_OPEN_BY_SERIAL_NUMBER,
            &mut handle,
        ))
    }?;
    if status == FT_OK {
        return Ok(handle);
    } else {
        return Err(Error::APIError(status));
    }
}

/// Open the device and return a handle which will be used for subsequent accesses.
///
/// Using [`FT_OPEN_BY_SERIAL_NUMBER`] allows an application to open a device that has the
/// specified Serial Number. Using [`FT_OPEN_BY_DESCRIPTION`] allows an application to open a
/// device that has the specified Product Description. Using [`FT_OPEN_BY_INDEX`] is a fall-back
/// option for instances where the devices connected to a machine do not have a unique Serial
/// Number or Product Description.
///
/// Returns [`FT_OK`] if successful, otherwise the return value is an
/// FT error code. See [`FT_Status`] for more information.
///
/// [`FT_OPEN_BY_SERIAL_NUMBER`]: libftd3xx_ffi::FT_OPEN_BY_SERIAL_NUMBER
/// [`FT_OPEN_BY_DESCRIPTION`]: libftd3xx_ffi::FT_OPEN_BY_DESCRIPTION
/// [`FT_OPEN_BY_INDEX`]: libftd3xx_ffi::FT_OPEN_BY_INDEX
///
/// # Example
///
/// ```no_run
/// use libftd3xx::functions::{create_by_description};
///
/// let handle = create_by_description("MyDescription").unwrap();
/// ```
pub fn create_by_description<S: Into<String>>(description: S) -> Result<FT_HANDLE> {
    //trace!("FT_Create(_)");
    let mut handle: FT_HANDLE = std::ptr::null_mut();
    let mut buffer: Vec<u8> = Vec::from(description.into());
    // Make sure our string is null terminated
    if !buffer.ends_with(&[0u8]) {
        buffer.push(0u8);
    }

    let status = unsafe {
        FT_Status::try_from(FT_Create(
            buffer.as_mut_ptr() as *mut std::ffi::c_void,
            FT_OPEN_BY_DESCRIPTION,
            &mut handle,
        ))
    }?;
    if status == FT_OK {
        return Ok(handle);
    } else {
        return Err(Error::APIError(status));
    }
}

/// Close an open device.
///
/// Returns [`FT_OK`] if successful, otherwise the return value is an
/// FT error code. See [`FT_Status`] for more information.
///
/// # Example
///
/// ```no_run
/// use libftd3xx::functions::{create_by_index, close};
///
/// let handle = create_by_index(0).unwrap();
/// close(handle).unwrap();
/// ```
pub fn close(handle: FT_HANDLE) -> Result<()> {
    //trace!("FT_Create(_)");

    let status = unsafe { FT_Status::try_from(FT_Close(handle)) }?;
    if status == FT_OK {
        return Ok(());
    } else {
        return Err(Error::APIError(status));
    }
}

// todo: FT_WritePipe
// todo: FT_ReadPipe
// todo: FT_WritePipeEx
// todo: FT_ReadPipeEx
// todo: FT_GetOverlappedResult
// todo: FT_GetOverlappedResult
// todo: FT_InitializeOverlapped
// todo: FT_ReleaseOverlapped
// todo: FT_SetStreamPipe
// todo: FT_ClearStreamPipe
// todo: FT_FlushPipe
// todo: FT_AbortPipe
// todo: FT_GetDeviceDescriptor
// todo: FT_GetConfigurationDescriptor
// todo: FT_GetInterfaceDescriptor
// todo: FT_GetPipeInformation
// todo: FT_GetStringDescriptor
// todo: FT_GetDescriptor
// todo: FT_ControlTransfer
// todo: FT_SetNotificationCallback
// todo: FT_ClearNotificationCallback
/// Returns the chip configuration.
///
/// Returns [`FT_OK`] if successful, otherwise the return value is an
/// FT error code. See [`FT_Status`] for more information.
///
/// # Example
///
/// ```no_run
/// use libftd3xx::functions::{create_by_index, get_chip_configuration};
///
/// let handle = create_by_index(0).unwrap();
/// let configuration = get_chip_configuration(handle).unwrap();
/// println!("{:#?}", configuration);
/// ```
///
/// # Sample Output
/// ```ignore
/// FT_60XCONFIGURATION {
///     VendorID: 2364,
///     ProductID: 4610,
///     StringDescriptors: [
///         ...
///     ],
///     bInterval: 9,
///     PowerAttributes: 160,
///     PowerConsumption: 900,
///     Reserved2: 0,
///     FIFOClock: 0,
///     FIFOMode: 1,
///     ChannelConfig: 2,
///     OptionalFeatureSupport: 0,
///     BatteryChargingGPIOConfig: 0,
///     FlashEEPROMDetection: 16,
///     MSIO_Control: 67584,
///     GPIO_Control: 0,
///     }
///```
pub fn get_chip_configuration(handle: FT_HANDLE) -> Result<FT_60XCONFIGURATION> {
    //trace!("FT_GetChipConfiguration(_)");
    let mut config = FT_60XCONFIGURATION::default();
    let status = unsafe {
        FT_Status::try_from(FT_GetChipConfiguration(
            handle,
            &mut config as *mut _ as *mut c_void,
        ))
    }?;
    if status == FT_OK {
        return Ok(config);
    } else {
        return Err(Error::APIError(status));
    }
}

/// This API can be used to modify the configurable fields in the chip configuration.
///
///  If config is None, the configuration will be reset to default configuration.
///
/// Returns [`FT_OK`] if successful, otherwise the return value is an
/// FT error code. See [`FT_Status`] for more information.
///
/// # Example
///
/// ```no_run
/// use libftd3xx::functions::{create_by_index, set_chip_configuration};
/// use libftd3xx_ffi::FT_60XCONFIGURATION;
///
/// // Open the first device
/// let handle = create_by_index(0).unwrap();
///
/// // Set some configuration parameters (I wouldn't default)
/// let configuration = FT_60XCONFIGURATION::default();
/// assert_eq!(set_chip_configuration(handle, Some(configuration)).is_ok(), true);
/// // Reset configuration:
/// assert_eq!(set_chip_configuration(handle, None).is_ok(), true);
/// ```
pub fn set_chip_configuration(
    handle: FT_HANDLE,
    config: Option<FT_60XCONFIGURATION>,
) -> Result<()> {
    //trace!("FT_SetChipConfiguration(_)");
    // pvConfiguration can be NULL, which will reset the configuration to defaults
    let config = match &config {
        Some(mut c) => &mut c as *mut _ as *mut c_void,
        None => std::ptr::null_mut() as *mut c_void,
    };
    let status = unsafe { FT_Status::try_from(FT_GetChipConfiguration(handle, config)) }?;
    if status == FT_OK {
        return Ok(());
    } else {
        return Err(Error::APIError(status));
    }
}
// todo: FT_GetFirmwareVersion
// todo: FT_ResetDevicePort
/// undocumented function
///
/// Returns [`FT_OK`] if successful, otherwise the return value is an
/// FT error code. See [`FT_Status`] for more information.
///
/// # Example
///
/// ```no_run
/// use libftd3xx::functions::{create_by_index, reset_device_port};
///
/// // Open the first device
/// let handle = create_by_index(0).unwrap();
/// assert_eq!(reset_device_port(handle).is_ok(), true);
/// ```
pub fn reset_device_port(handle: FT_HANDLE) -> Result<()> {
    //trace!("FT_ResetDevicePort(_)");

    let status = unsafe { FT_Status::try_from(FT_ResetDevicePort(handle)) }?;
    if status == FT_OK {
        return Ok(());
    } else {
        return Err(Error::APIError(status));
    }
}
// todo: FT_CycleDevicePort
/// Power cycles the device port. This causes the device to be re-enumerated by the host
/// system.
///
/// Returns [`FT_OK`] if successful, otherwise the return value is an
/// FT error code. See [`FT_Status`] for more information.
///
/// # Example
///
/// ```no_run
/// use libftd3xx::functions::{create_by_index, cycle_device_port};
/// // Open the first device
/// let handle = create_by_index(0).unwrap();
/// assert_eq!(cycle_device_port(handle).is_ok(), true);
/// ```
pub fn cycle_device_port(handle: FT_HANDLE) -> Result<()> {
    //trace!("FT_CycleDevicePort(_)");

    let status = unsafe { FT_Status::try_from(FT_CycleDevicePort(handle)) }?;
    if status == FT_OK {
        return Ok(());
    } else {
        return Err(Error::APIError(status));
    }
}

// todo: FT_CreateDeviceInfoList
// todo: FT_GetDeviceInfoList
// todo: FT_GetDeviceInfoDetail
// todo: FT_ListDevices
// todo: FT_IsDevicePath
// todo: FT_GetDriverVersion
// todo: FT_GetLibraryVersion
// todo: FT_SetPipeTimeout
// todo: FT_EnableGPIO
// todo: FT_WriteGPIO
// todo: FT_ReadGPIO
// todo: FT_SetGPIOPull

#[cfg(test)]
mod tests {
    use super::*;
    use crate::functions::Error::APIError;
    use std::ffi::CStr;
    use std::{thread, time};

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

    // boilerplate function to simplify all the tests
    #[cfg(feature = "hardware_tests")]
    fn get_first_device() -> FT_HANDLE {
        let mut handle = std::ptr::null_mut();
        loop {
            // Open the handle, sometimes we get some errors so lets retry here...
            handle = match create_by_index(0) {
                Ok(h) => h,
                Err(APIError(FT_DEVICE_NOT_OPENED))
                | Err(APIError(FT_DEVICE_NOT_FOUND))
                | Err(APIError(FT_OTHER_ERROR)) => continue,
                Err(e) => panic!("create_by_index(0) failed: {e}"),
            };
            break;
        }
        handle
        /*
        // Grab the first device
        let mut device_count = create_device_info_list().unwrap();
        assert_eq!(
            device_count >= 1,
            true,
            "Expected at least one device, got {device_count}"
        );
        let mut sn = String::new();
        // There is a bug in the FTD3XX library that randomly doesn't return the serial number, lets loop here until we get one
        loop {
            let info_list: Vec<libftd3xx_ffi::_FT_DEVICE_LIST_INFO_NODE> = get_device_info_list(&mut device_count).unwrap();
            // convert the serial number from a cstr to a String
            let cstr_sn = unsafe { CStr::from_ptr(info_list[0].SerialNumber.as_ptr()) };
            sn = String::from_utf8_lossy(cstr_sn.to_bytes()).to_string();
            println!("SN: {sn} - {}", sn.len());
            if sn.len() == 0 {
                println!("Serial number isn't valid!");
                continue;
            } else {
                println!("Serial number is valid! {sn} - {}", sn.len());
                break;
            }
        }
        assert!(!sn.is_empty());
        let mut handle = std::ptr::null_mut();
        loop {
            // Open the handle, sometimes we get FT_DEVICE_NOT_OPENED so lets retry here...
            handle = match create_by_serial_number(&sn) {
                Ok(h) => h,
                Err(APIError(FT_DEVICE_NOT_OPENED)) | Err(APIError(FT_DEVICE_NOT_FOUND)) => {
                    continue
                }
                Err(e) => panic!("create_by_serial_number({sn}) failed: {e}"),
            };
            break;
        }
        handle
        */
    }

    //#[cfg(not(feature = "hardware_tests"))]
    #[test]
    fn test_get_driver_version_invalid() {
        use crate::functions::Error::APIError;

        let result = get_driver_version(std::ptr::null_mut());
        assert_eq!(result, Err(APIError(FT_INVALID_HANDLE)));
    }

    #[cfg(feature = "hardware_tests")]
    #[test]
    fn test_get_device_info_list() {
        let device_count = create_device_info_list().unwrap();
        assert_eq!(device_count >= 1, true);
        let mut num_devices = device_count.clone();
        let info_list = get_device_info_list(&mut num_devices).unwrap();
        assert_eq!(info_list.len() >= 1, true);
        assert_eq!(num_devices, device_count);
    }

    #[cfg(feature = "hardware_tests")]
    #[test]
    fn test_get_driver_version() {
        // Grab the first device
        let handle = get_first_device();

        let result = get_driver_version(handle);
        assert_eq!(result.is_ok(), true);
        assert!(close(handle).is_ok());
    }

    #[cfg(feature = "hardware_tests")]
    #[test]
    fn test_get_chip_configuration() {
        // Grab the first device
        let handle = get_first_device();

        let result = get_chip_configuration(handle);
        assert_eq!(result.is_ok(), true);
        assert!(close(handle).is_ok());
        //println!("{:#?}", result.unwrap())
    }

    #[cfg(feature = "hardware_tests")]
    #[test]
    fn test_set_chip_configuration() {
        // Grab the first device
        let handle = get_first_device();
        // Read the configuration
        let result = get_chip_configuration(handle);
        assert_eq!(result.is_ok(), true);
        let config = result.unwrap();
        // Set the configuration
        let result = set_chip_configuration(handle, Some(config));
        assert_eq!(result.is_ok(), true);

        assert!(close(handle).is_ok());
        //println!("{:#?}", result.unwrap())
    }

    #[cfg(feature = "hardware_tests")]
    #[test]
    fn test_cycle_device_port() {
        // Grab the first device
        let handle = get_first_device();
        let result = cycle_device_port(handle);
        assert!(result.is_ok());
        assert!(close(handle).is_ok());
    }

    #[cfg(feature = "hardware_tests")]
    #[test]
    fn test_reset_device_port() {
        // undocumented function, might need to disable if its causing issues here.
        // Grab the first device
        let handle = get_first_device();
        let result = reset_device_port(handle);
        assert!(result.is_ok());
        assert!(close(handle).is_ok());
    }
}
