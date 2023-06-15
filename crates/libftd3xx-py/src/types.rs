use libftd3xx_ffi::DWORD;
use libftd3xx_ffi::UCHAR;
use libftd3xx_ffi::ULONG;
use libftd3xx_ffi::USHORT;
use pyo3::create_exception;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use std::ffi::CStr;
use std::fmt;
use std::sync::{Arc, Mutex};

use ::libftd3xx as ftd3xx;
use ftd3xx::types::Error as FtError;
use libftd3xx_ffi::{prelude::*, FT_DEVICE_LIST_INFO_NODE, FT_60XCONFIGURATION};

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

define_basic_py_object_no_new!(Ft60xConfiguration, FT_60XCONFIGURATION);

impl Ft60xConfiguration {
    fn new() -> Self {
        Self {
            0: Arc::new(Mutex::new(FT_60XCONFIGURATION::default())),
        }
    }

    pub fn from(data: FT_60XCONFIGURATION) -> Self {
        Self {
            0: Arc::new(Mutex::new(data)),
        }
    }
}

#[pymethods]
impl Ft60xConfiguration {
    #[new]
    fn py_new() -> Self {
        Self::new()
    }

    #[getter(VendorID)]
    fn get_vendor_id(&self) -> PyResult<USHORT> {
        let config = *self.0.lock().unwrap();
        Ok(config.VendorID)
    }
    #[setter(VendorID)]
    fn set_vendor_id(&mut self, value: USHORT) -> PyResult<()> {
        let mut config = *self.0.lock().unwrap();
        config.VendorID = value;
        Ok(())
    }

    #[getter(ProductID)]
    fn get_product_id(&self) -> PyResult<USHORT> {
        let config = *self.0.lock().unwrap();
        Ok(config.ProductID)
    }
    #[setter(ProductID)]
    fn set_product_id(&mut self, value: USHORT) -> PyResult<()> {
        let mut config = *self.0.lock().unwrap();
        config.ProductID = value;
        Ok(())
    }

    // StringDescriptors: [UCHAR; 128usize],
    #[getter(StringDescriptors)]
    fn get_string_descriptors(&self) -> PyResult<[UCHAR; 128usize]> {
        let config = *self.0.lock().unwrap();
        Ok(config.StringDescriptors)
    }
    #[setter(StringDescriptors)]
    fn set_string_descriptors(&mut self, value: [UCHAR; 128usize]) -> PyResult<()> {
        let mut config = *self.0.lock().unwrap();
        config.StringDescriptors = value;
        Ok(())
    }

    #[getter(bInterval)]
    fn get_b_interval(&self) -> PyResult<UCHAR> {
        let config = *self.0.lock().unwrap();
        Ok(config.bInterval)
    }
    #[setter(bInterval)]
    fn set_b_interval(&mut self, value: UCHAR) -> PyResult<()> {
        let mut config = *self.0.lock().unwrap();
        config.bInterval = value;
        Ok(())
    }

    #[getter(PowerAttributes)]
    fn get_power_attributes(&self) -> PyResult<UCHAR> {
        let config = *self.0.lock().unwrap();
        Ok(config.PowerAttributes)
    }
    #[setter(PowerAttributes)]
    fn set_power_attributes(&mut self, value: UCHAR) -> PyResult<()> {
        let mut config = *self.0.lock().unwrap();
        config.PowerAttributes = value;
        Ok(())
    }

    #[getter(PowerConsumption)]
    fn get_power_consumption(&self) -> PyResult<USHORT> {
        let config = *self.0.lock().unwrap();
        Ok(config.PowerConsumption)
    }
    #[setter(PowerConsumption)]
    fn set_power_consumption(&mut self, value: USHORT) -> PyResult<()> {
        let mut config = *self.0.lock().unwrap();
        config.PowerConsumption = value;
        Ok(())
    }

    #[getter(Reserved2)]
    fn get_reserved2(&self) -> PyResult<UCHAR> {
        let config = *self.0.lock().unwrap();
        Ok(config.Reserved2)
    }
    #[setter(Reserved2)]
    fn set_reserved2(&mut self, value: UCHAR) -> PyResult<()> {
        let mut config = *self.0.lock().unwrap();
        config.Reserved2 = value;
        Ok(())
    }

    #[getter(FIFOClock)]
    fn get_fifo_clock(&self) -> PyResult<UCHAR> {
        let config = *self.0.lock().unwrap();
        Ok(config.FIFOClock)
    }
    #[setter(FIFOClock)]
    fn set_fifo_clock(&mut self, value: UCHAR) -> PyResult<()> {
        let mut config = *self.0.lock().unwrap();
        config.FIFOClock = value;
        Ok(())
    }

    #[getter(FIFOMode)]
    fn get_fifo_mode(&self) -> PyResult<UCHAR> {
        let config = *self.0.lock().unwrap();
        Ok(config.FIFOMode)
    }
    #[setter(FIFOMode)]
    fn set_fifo_mode(&mut self, value: UCHAR) -> PyResult<()> {
        let mut config = *self.0.lock().unwrap();
        config.FIFOMode = value;
        Ok(())
    }

    #[getter(ChannelConfig)]
    fn get_channel_config(&self) -> PyResult<UCHAR> {
        let config = *self.0.lock().unwrap();
        Ok(config.ChannelConfig)
    }
    #[setter(ChannelConfig)]
    fn set_channel_config(&mut self, value: UCHAR) -> PyResult<()> {
        let mut config = *self.0.lock().unwrap();
        config.ChannelConfig = value;
        Ok(())
    }

    #[getter(OptionalFeatureSupport)]
    fn get_optional_feature_support(&self) -> PyResult<USHORT> {
        let config = *self.0.lock().unwrap();
        Ok(config.OptionalFeatureSupport)
    }
    #[setter(OptionalFeatureSupport)]
    fn set_optional_feature_support(&mut self, value: USHORT) -> PyResult<()> {
        let mut config = *self.0.lock().unwrap();
        config.OptionalFeatureSupport = value;
        Ok(())
    }

    #[getter(BatteryChargingGPIOConfig)]
    fn get_battery_charging_gpio_config(&self) -> PyResult<UCHAR> {
        let config = *self.0.lock().unwrap();
        Ok(config.BatteryChargingGPIOConfig)
    }
    #[setter(BatteryChargingGPIOConfig)]
    fn set_battery_charging_gpio_config(&mut self, value: UCHAR) -> PyResult<()> {
        let mut config = *self.0.lock().unwrap();
        config.BatteryChargingGPIOConfig = value;
        Ok(())
    }

    #[getter(FlashEEPROMDetection)]
    fn get_flash_eeprom_detection(&self) -> PyResult<UCHAR> {
        let config = *self.0.lock().unwrap();
        Ok(config.FlashEEPROMDetection)
    }
    #[setter(FlashEEPROMDetection)]
    fn set_flash_eeprom_detection(&mut self, value: UCHAR) -> PyResult<()> {
        let mut config = *self.0.lock().unwrap();
        config.FlashEEPROMDetection = value;
        Ok(())
    }

    #[getter(MSIO_Control)]
    fn get_msio_control(&self) -> PyResult<ULONG> {
        let config = *self.0.lock().unwrap();
        Ok(config.MSIO_Control)
    }
    #[setter(MSIO_Control)]
    fn set_msio_control(&mut self, value: ULONG) -> PyResult<()> {
        let mut config = *self.0.lock().unwrap();
        config.MSIO_Control = value;
        Ok(())
    }

    #[getter(GPIO_Control)]
    fn get_gpio_control(&self) -> PyResult<ULONG> {
        let config = *self.0.lock().unwrap();
        Ok(config.GPIO_Control)
    }
    #[setter(GPIO_Control)]
    fn set_gpio_control(&mut self, value: ULONG) -> PyResult<()> {
        let mut config = *self.0.lock().unwrap();
        config.GPIO_Control = value;
        Ok(())
    }    
}
