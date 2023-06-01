use libftd3xx_ffi::prelude::*;

pub mod types;

use types::{Error, Result, Version};


/// Returns the version of the underlying C library.
///
/// **Note**: The documentation says this function is only supported on Windows
/// but it seems to work correctly on Linux.
///
/// # Example
///
/// ```no_run
/// use libftd3xx::library_version;
///
/// let version = library_version()?;
/// println!("libftd3xx C library version: {}", version);
/// # Ok::<(), libftd3xx::FtStatus>(())
/// ```
pub fn library_version() -> Result<Version> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_version() {
        let result = library_version();
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
}
