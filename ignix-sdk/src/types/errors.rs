use core::ops::Not;

// SPDX-License-Identifier: GPL-3.0-only
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Status(pub usize);
/* Those comments and codes are directly extracted from the UEFI spec 2.11 page 1984 to 1986
 * There is also some custom errors made for this bootloaders */
#[allow(unused)]
impl Status {
    const ERROR_BIT: usize = 1 << (usize::BITS - 1);
    const CUSTOM_ERROR_BIT: usize = Self::ERROR_BIT | 0x1000;

    /// The operation completed successfully.
    pub const SUCCESS: Self = Status(0);

    /// The string contained characters that could not be rendered and were skipped.
    pub const WARN_UNKNOWN_GLYPH: Self = Status(1);
    /// The handle was closed, but the file was not deleted.
    pub const WARN_DELETE_FAILURE: Self = Status(2);
    /// The handle was closed, but the data to the file was not flushed properly.
    pub const WARN_WRITE_FAILURE: Self = Status(3);
    /// The resulting buffer was too small, and the data was truncated.
    pub const WARN_BUFFER_TOO_SMALL: Self = Status(4);
    /// The data has not been updated within the timeframe set by local policy.
    pub const WARN_STALE_DATA: Self = Status(5);
    /// The resulting buffer contains UEFI-compliant file system.
    pub const WARN_FILE_SYSTEM: Self = Status(6);
    /// The operation will be processed across a system reset.
    pub const WARN_RESET_REQUIRED: Self = Status(7);

    /// The image failed to load.
    pub const LOAD_ERROR: Self = Status(Self::ERROR_BIT | 1);
    /// A parameter was incorrect.
    pub const INVALID_PARAMETER: Self = Status(Self::ERROR_BIT | 2);
    /// The operation isn't supported.
    pub const UNSUPPORTED: Self = Status(Self::ERROR_BIT | 3);
    /// The buffer wasn't the proper size for the request.
    pub const BAD_BUFFER_SIZE: Self = Status(Self::ERROR_BIT | 4);
    /// The buffer isn't large enough to hold the requested data.
    pub const BUFFER_TOO_SMALL: Self = Status(Self::ERROR_BIT | 5);
    /// There is no data pending upon return.
    pub const NOT_READY: Self = Status(Self::ERROR_BIT | 6);
    /// The physical device reported an error while attempting the operation.
    pub const DEVICE_ERROR: Self = Status(Self::ERROR_BIT | 7);
    /// The device cannot be written to.
    pub const WRITE_PROTECTED: Self = Status(Self::ERROR_BIT | 8);
    /// A resource has run out.
    pub const OUT_OF_RESOURCES: Self = Status(Self::ERROR_BIT | 9);
    /// An inconsistency was detected on the FS causing the operation to fail.
    pub const VOLUME_CORRUPTED: Self = Status(Self::ERROR_BIT | 10);
    /// There is no more space left on the FS.
    pub const VOLUME_FULL: Self = Status(Self::ERROR_BIT | 11);
    /// The device doesn't contain any medium to perform the operation.
    pub const NO_MEDIA: Self = Status(Self::ERROR_BIT | 12);
    /// The medium in the device has changed since the last access.
    pub const MEDIA_CHANGED: Self = Status(Self::ERROR_BIT | 13);
    /// The item wasn't found.
    pub const NOT_FOUND: Self = Status(Self::ERROR_BIT | 14);
    /// Access was denied.
    pub const ACCESS_DENIED: Self = Status(Self::ERROR_BIT | 15);
    /// The server wasn't found or didn't respond to the request.
    pub const NO_RESPONSE: Self = Status(Self::ERROR_BIT | 16);
    /// A mapping device doesn't exists.
    pub const NO_MAPPING: Self = Status(Self::ERROR_BIT | 17);
    /// The timeout has expired.
    pub const TIMEOUT: Self = Status(Self::ERROR_BIT | 18);
    /// The protocol has not been started.
    pub const NOT_STARTED: Self = Status(Self::ERROR_BIT | 19);
    /// The protocol has already been started.
    pub const ALREADY_STARTED: Self = Status(Self::ERROR_BIT | 20);
    /// The operation was aborted.
    pub const ABORTED: Self = Status(Self::ERROR_BIT | 21);
    /// An ICMP error occurred during the network operation.
    pub const ICMP_ERROR: Self = Status(Self::ERROR_BIT | 22);
    /// A TFTP error occurred during the network operation.
    pub const TFTP_ERROR: Self = Status(Self::ERROR_BIT | 23);
    /// A protocol error occurred during the network operation.
    pub const PROTOCOL_ERROR: Self = Status(Self::ERROR_BIT | 24);
    /// The function encountered an internal version that was incompatible.
    pub const INCOMPATIBLE_VERSION: Self = Status(Self::ERROR_BIT | 25);
    /// The function wasn't performed due to a security violation.
    pub const SECURITY_VIOLATION: Self = Status(Self::ERROR_BIT | 26);
    /// A CRC error was detected.
    pub const CRC_ERROR: Self = Status(Self::ERROR_BIT | 27);
    /// Beginning or end of media was reached.
    pub const END_OF_MEDIA: Self = Status(Self::ERROR_BIT | 28);
    /// The end of the file was reached.
    pub const END_OF_FILE: Self = Status(Self::ERROR_BIT | 31);
    /// The language specified was invalid.
    pub const INVALID_LANGUAGE: Self = Status(Self::ERROR_BIT | 32);
    /// The security status of the data is unknown or compromised.
    pub const COMPROMISED_DATA: Self = Status(Self::ERROR_BIT | 33);
    /// There is an address conflict address allocation.
    pub const IP_ADDRESS_CONFLICT: Self = Status(Self::ERROR_BIT | 34);
    /// A HTTP error occurred during the network operation.
    pub const HTTP_ERROR: Self = Status(Self::ERROR_BIT | 35);
    // This is the start of the custom errors for ignix
    /// System Table pointer missing.
    pub const ST_POINTER_MISSING: Self = Status(Self::CUSTOM_ERROR_BIT | 1);
    /// Boot Services table pointer missing.
    pub const BST_POINTER_MISSING: Self = Status(Self::CUSTOM_ERROR_BIT | 2);
    /// Runtime Services table pointer missing.
    pub const RST_POINTER_MISSING: Self = Status(Self::CUSTOM_ERROR_BIT | 3);
    /// Handle device is null
    pub const HANDLE_DEVICE_IS_NULL: Self = Status(Self::CUSTOM_ERROR_BIT | 4);
}
#[allow(unused)]
impl Status {
    #[inline]
    #[must_use] // This is just so the compiler can be a bitch with you if you forget to do it lmao
    pub fn is_success(self) -> bool {
        self == Status::SUCCESS
    }
    #[inline]
    #[must_use]
    pub fn is_warning(self) -> bool {
        self != Self::SUCCESS && (self.0 & Self::ERROR_BIT == 0)
    }

    #[inline]
    #[must_use]
    pub const fn is_error(self) -> bool {
        (self.0 & Self::ERROR_BIT) != 0
    }

    pub fn context(self, func: &'static str) -> IgnixError {
        if func.is_empty() {
            return IgnixError {
                status: self,
                func: "unknown",
            };
        }
        IgnixError { status: self, func }
    }
}

impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::SUCCESS => write!(f, "FW: SUCCESS"),

            Self::WARN_UNKNOWN_GLYPH => write!(f, "FW:WARN_UNKNOWN_GLYPH"),
            Self::WARN_DELETE_FAILURE => write!(f, "FW: WARN_DELETE_FAILURE"),
            Self::WARN_WRITE_FAILURE => write!(f, "FW: WARN_WRITE_FAILURE"),
            Self::WARN_BUFFER_TOO_SMALL => write!(f, "FW: WARN_BUFFER_TOO_SMALL"),
            Self::WARN_STALE_DATA => write!(f, "FW: WARN_STALE_DATA"),
            Self::WARN_FILE_SYSTEM => write!(f, "FW: WARN_FILE_SYSTEM"),
            Self::WARN_RESET_REQUIRED => write!(f, "FW: WARN_RESET_REQUIRED"),

            Self::LOAD_ERROR => write!(f, "FW: LOAD_ERROR"),
            Self::INVALID_PARAMETER => write!(f, "FW: INVALID_PARAMETER"),
            Self::UNSUPPORTED => write!(f, "FW: UNSUPPORTED"),
            Self::BAD_BUFFER_SIZE => write!(f, "FW: BAD_BUFFER_SIZE"),
            Self::BUFFER_TOO_SMALL => write!(f, "FW: BUFFER_TOO_SMALL"),
            Self::NOT_READY => write!(f, "FW: NOT_READY"),
            Self::DEVICE_ERROR => write!(f, "FW: DEVICE_ERROR"),
            Self::WRITE_PROTECTED => write!(f, "FW: WRITE_PROTECTED"),
            Self::OUT_OF_RESOURCES => write!(f, "FW: OUT_OF_RESOURCES"),
            Self::VOLUME_CORRUPTED => write!(f, "FW: VOLUME_CORRUPTED"),
            Self::VOLUME_FULL => write!(f, "FW: VOLUME_FULL"),
            Self::NO_MEDIA => write!(f, "FW: NO_MEDIA"),
            Self::MEDIA_CHANGED => write!(f, "FW: MEDIA_CHANGED"),
            Self::NOT_FOUND => write!(f, "FW: NOT_FOUND"),
            Self::ACCESS_DENIED => write!(f, "FW: ACCESS_DENIED"),
            Self::NO_RESPONSE => write!(f, "FW: NO_RESPONSE"),
            Self::NO_MAPPING => write!(f, "FW: NO_MAPPING"),
            Self::TIMEOUT => write!(f, "FW: TIMEOUT"),
            Self::NOT_STARTED => write!(f, "FW: NOT_STARTED"),
            Self::ALREADY_STARTED => write!(f, "FW: ALREADY_STARTED"),
            Self::ABORTED => write!(f, "FW: ABORTED"),
            Self::ICMP_ERROR => write!(f, "FW: ICMP_ERROR"),
            Self::TFTP_ERROR => write!(f, "FW: TFTP_ERROR"),
            Self::PROTOCOL_ERROR => write!(f, "FW: PROTOCOL_ERROR"),
            Self::INCOMPATIBLE_VERSION => write!(f, "FW: INCOMPATIBLE_VERSION"),
            Self::SECURITY_VIOLATION => write!(f, "FW: SECURITY_VIOLATION"),
            Self::CRC_ERROR => write!(f, "FW: CRC_ERROR"),
            Self::END_OF_MEDIA => write!(f, "FW: END_OF_MEDIA"),
            Self::END_OF_FILE => write!(f, "FW: END_OF_FILE"),
            Self::INVALID_LANGUAGE => write!(f, "FW: INVALID_LANGUAGE"),
            Self::COMPROMISED_DATA => write!(f, "FW: COMPROMISED_DATA"),
            Self::IP_ADDRESS_CONFLICT => write!(f, "FW: IP_ADDRESS_CONFLICT"),
            Self::HTTP_ERROR => write!(f, "FW: HTTP_ERROR"),
            Self::ST_POINTER_MISSING => write!(f, "IGNIX: ST_POINTER_MISSING"),
            Self::BST_POINTER_MISSING => write!(f, "IGNIX: BST_POINTER_MISSING"),
            Self::RST_POINTER_MISSING => write!(f, "IGNIX: RST_POINTER_MISSING"),
            Self::HANDLE_DEVICE_IS_NULL => write!(f, "IGNIX: HANDLE_DEVICE_IS_NULL"),
            _ => write!(f, "What the fuck did you do Status(0x{:X})", self.0),
        }
    }
}

pub struct IgnixError {
    pub status: Status,
    pub func: &'static str,
}

impl core::fmt::Debug for IgnixError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?} in {} function.", self.status, self.func)
    }
}

impl core::fmt::Display for IgnixError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}
