#![deny(missing_docs, unsafe_code)]
///! Contains higher level objects and or constant values.
use libftd3xx_ffi::prelude::*;
use core::fmt;

/// Errors associated with this library
#[derive(Debug)]
pub enum Error {
    /// Low level API error directly from the FTD3xx Library
    APIError(FT_Status),
    /// Low level Critical API error, this is something that would normally "panic"
    CriticalAPIError(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::APIError(s) => write!(f, "API Error: {:#?}", s),
            Self::CriticalAPIError(s) => write!(f, "Critical API Error: {:#?}", s),
        }
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::CriticalAPIError(value.to_string())
    }
}

/// Generic crate Result object
pub type Result<T> = std::result::Result<T, Error>;

/// D3XX version structure.
///
/// A version number contains a major version number, minor version and build/SVN version.
/// Byte 0 and 1 (least significant) holds the build/SVN version. Byte 2 holds the minor
/// version. Byte 3 holds the major version.
///
/// This is returned by [`get_library_version`] and [`driver_version`].
///
/// [`get_library_version`]: crate::get_library_version
/// [`driver_version`]: crate::FtdiCommon::driver_version
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Version {
    /// Major version.
    pub major: u8,
    /// Minor version.
    pub minor: u8,
    /// Build number.
    pub build: u16,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.build)
    }
}

impl Version {
    /// Create a new version structure from decimal values.
    ///
    /// # Example
    ///
    /// ```
    /// use libftd2xx::Version;
    ///
    /// let version = Version::new(3, 1, 15);
    /// assert_eq!(
    ///     version,
    ///     Version {
    ///         major: 3,
    ///         minor: 1,
    ///         build: 15
    ///     }
    /// );
    /// ```
    pub const fn new(major: u8, minor: u8, build: u16) -> Version {
        Version {
            major,
            minor,
            build,
        }
    }

    /// Create a new version structure from the raw C-API value.
    ///
    /// Byte 0 and 1 (least significant) holds the build/SVN version. Byte 2 holds the minor
    /// version. Byte 3 holds the major version.
    ///
    /// # Example
    ///
    /// ```
    /// use libftd2xx::Version;
    ///
    /// let version = Version::with_raw(0x00030115);
    /// assert_eq!(version, Version::new(3, 1, 15));
    /// ```
    pub fn with_raw(value: u32) -> Version {
        Version::new(
            ((value >> 24) & 0xFF) as u8,
            ((value >> 16) & 0xFF) as u8,
            (value & 0xFFFF) as u16,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_version {
        ($name:ident, ($a:expr, $b:expr, $c:expr), ($d:expr, $e:expr, $f:expr)) => {
            #[test]
            fn $name() {
                let big = Version::new($a, $b, $c);
                let little = Version::new($d, $e, $f);
                assert!(big > little);
                assert!(little < big);
            }
        };
    }

    test_version!(case_1, (0, 0, 1), (0, 0, 0));
    test_version!(case_2, (0, 1, 0), (0, 0, 0));
    test_version!(case_3, (1, 0, 0), (0, 0, 0));
    test_version!(case_4, (2, 2, 2), (1, 1, 1));
    test_version!(case_5, (255, 255, 255), (254, 255, 255));
    test_version!(case_6, (1, 0, 0), (0, 255, 255));
    test_version!(case_7, (13, 255, 0), (13, 254, 255));

    #[test]
    fn display() {
        assert_eq!(
            format!("{}", Version::with_raw(0x00030115)),
            String::from("3.1.15")
        )
    }
}