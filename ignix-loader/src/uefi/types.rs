/*
 * Copyright (C) 2026 Flamitsu
 *
 * This file is part of Ignix.
 *
 * Ignix is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * Ignix is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Ignix.  If not, see <https://www.gnu.org/licenses/>.
 */
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Status(pub usize);
// Those comments and codes are directly extracted from the UEFI spec 2.11 page 1984 to 1986
#[allow(unused)]
impl Status {
    pub const ERROR_BIT: usize = 1 << (usize::BITS - 1);
    
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
}
