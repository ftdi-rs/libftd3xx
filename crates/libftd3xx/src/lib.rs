use libftd3xx_ffi::prelude::*;

type Result<T> = std::result::Result<T, FT_STATUS>;

/// Returns the version of the underlying C library.
///
/// **Note**: The documentation says this function is only supported on Windows
/// but it seems to work correctly on Linux.
///
/// # Example
///
/// ```no_run
/// use libftd2xx::library_version;
///
/// let version = library_version()?;
/// println!("libftd2xx C library version: {}", version);
/// # Ok::<(), libftd2xx::FtStatus>(())
/// ```
pub fn library_version() -> Result<u32> {
    let mut version: u32 = 0;
    //trace!("FT_GetLibraryVersion(_)");
    let status: FT_STATUS = unsafe { FT_GetLibraryVersion(&mut version) };
    if status == FT_OK as FT_STATUS {
        return Ok(version);
    } else {
        return Err(status);
    }
}
