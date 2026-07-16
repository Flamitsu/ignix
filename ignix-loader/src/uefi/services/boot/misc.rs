// SPDX-License-Identifier: GPL-3.0-only
use crate::uefi::{
    table::{SystemTable, boot::BootServicesWrapper},
    types::{Guid, Status, Table},
};
use core::{ffi::c_void, time::Duration};
impl BootServicesWrapper {
    /// Sets the system's watchdog timer.
    /// Timeout is the number of seconds the watchdog timer will be set up to. Value 0 disables it.
    /// WatchdogCode is a numeric code to log on a watchdog timer timeout event. The firmware
    /// saves 0x0000..0xFFFF. Loaders may use other timeout codes.
    /// RETURN CODES:
    ///
    /// EFI_SUCCESS The timeout has been set.
    /// EFI_INVALID_PARAMETER The supplied WatchdogCode is invalid.
    /// EFI_UNSUPPORTED The system does not have a watchdog timer.
    /// EFI_DEVICE_ERROR The watch dog timer could not be programmed due to a hardware error.
    pub fn set_watchdog_timer(&self, timeout: Duration, watchdog_code: u64) -> Result<(), Status> {
        let Some(function) = self.get_method() else {
            Err(Status::NOT_FOUND)?
        };
        let status = unsafe {
            (function.set_watch_dog_timer)(
                timeout.as_secs().try_into().unwrap(),
                watchdog_code,
                0,
                core::ptr::null(),
            )
        };
        if status.is_error() {
            Err(status)?
        }
        Ok(())
    }

    /// Stalls the system the seconds (converted internally to microseconds)
    pub fn stall(&self, duration: Duration) -> Result<(), Status> {
        let Some(function) = self.get_method() else {
            Err(Status::NOT_FOUND)?
        };
        let microseconds = duration.as_micros() as usize;
        let status = unsafe { (function.stall)(microseconds) };

        if status.is_error() {
            Err(status)?
        }

        Ok(())
    }

    /// Copies the contents of one buffer to another.
    /// The following rules can be used to guarantee the correct behavior:
    /// 1. If Destination and Source are identical, then no operation should be performed.
    /// 2. If Destination > Source and Destination < ( Source + Length ), then the data should be
    /// copied from the Source buffer to the Destination buffer starting from the end of the buffers
    /// and working toward the beginning of the buffers.
    /// 3. Otherwise, the data should be copied from the Source buffer to the Destination buffer
    /// starting from the beginning of the buffers and working toward the end of the buffers.
    pub fn copy_mem(&self, dest: &mut [u8], src: &[u8]) -> Result<(), Status> {
        let Some(function) = self.get_method() else {
            Err(Status::NOT_FOUND)?
        };
        if dest.len() > src.len() {
            Err(Status::INVALID_PARAMETER)?
        }
        unsafe {
            (function.copy_mem)(
                dest.as_mut_ptr() as *const c_void,
                src.as_ptr() as *const c_void,
                src.len(),
            )
        }
        Ok(())
    }
    /// Fills a buffer with a specified value.
    pub fn set_mem<const N: usize>(&self, mut buffer: [u8; N], value: u8) -> Result<(), Status> {
        let Some(function) = self.get_method() else {
            Err(Status::NOT_FOUND)?
        };
        unsafe { (function.set_mem)(buffer.as_mut_ptr() as *const c_void, buffer.len(), value) }
        Ok(())
    }

    /// Returns a monotically increasing count for the platform
    /// The platform's monotonic count is made in 2 parts, the high 32 bits and the low
    /// 32 bits.
    /// The low 32 bits resets to zero on every system reset.
    /// The high 32-bit value is nonvolatile and is increased by one on whenever the system resets
    /// or the low 32-bit counter overflows
    pub fn get_next_monotonic_count(&self, count: u64) -> Result<u64, Status> {
        let Some(function) = self.get_method() else {
            Err(Status::NOT_FOUND)?
        };
        // I did this so the variable itself isn't modified, it's modified value its a
        // returned value.
        let mut shadow = count;
        let status = unsafe { (function.get_next_monotonic_count)(&mut shadow) };
        if status.is_error() {
            Err(status)?
        }
        Ok(shadow)
    }

    /// Adds, update or removes a configuration table entry from the EFI System Table
    ///
    /// RETURN CODES:
    /// EFI_SUCCESS The (Guid, Table) pair was added, updated, or removed.
    /// EFI_INVALID_PARAMETER Guid is NULL.
    /// EFI_NOT_FOUND An attempt was made to delete a nonexistent entry.
    /// EFI_OUT_OF_RESOURCES There is not enough memory available to complete the operation
    pub fn install_configuration_table<T: Table>(
        &self,
        guid: *const Guid,
        table: Option<*const T>,
    ) -> Result<(), Status> {
        let Some(function) = self.get_method() else {
            Err(Status::NOT_FOUND)?
        };

        let table_ptr = match table {
            Some(ptr) => ptr as *const c_void,
            None => core::ptr::null(),
        };

        let status = unsafe { (function.install_configuration_table)(guid, table_ptr) };

        if status.is_error() {
            Err(status)?
        }

        Ok(())
    }
    /// This function computes the 32-bit CRC for the data buffer specified by Data and *DataSize.
    /// If the 32-bit CRC is computed, then it is returned in Crc32 and EFI_SUCCESS is returned.
    ///
    /// RETURN:
    /// EFI_SUCCESS The 32-bit CRC was computed for the data buffer and returned as u32.
    /// EFI_INVALID_PARAMETER Data is NULL.
    /// EFI_INVALID_PARAMETER Crc32 is NULL.
    /// EFI_INVALID_PARAMETER DataSize is 0
    pub fn calculate_crc32<const N: usize>(&self, buffer: [u8; N]) -> Result<u32, Status> {
        let Some(function) = self.get_method() else {
            Err(Status::NOT_FOUND)?
        };
        let mut crc32: u32 = 0;
        let status = unsafe {
            (function.calculate_crc32)(buffer.as_ptr() as *const c_void, buffer.len(), &mut crc32)
        };
        if status.is_error() {
            Err(status)?
        }
        Ok(crc32)
    }
}
