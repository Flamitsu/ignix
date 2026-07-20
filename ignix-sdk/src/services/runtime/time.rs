use core::{ffi::c_void, mem::zeroed, ptr::null};

use crate::{
    table::runtime::RuntimeServicesWrapper,
    types::{Boolean, IgnixError, Status, Time, TimeCapabilities, TimeStruct, WakeupTime},
};
impl RuntimeServicesWrapper {
    /// Returns the current date and time information, with the time-keeping capabilities of the hw
    /// platform
    ///
    /// RETURN CODES:
    /// EFI_INVALID_PARAMETER Time is NULL.
    /// EFI_DEVICE_ERROR The time could not be retrieved due to a hardware error.
    /// EFI_UNSUPPORTED This call is not supported by this platform at the time the call is made.
    /// The platform should describe this runtime service as unsupported at runtime via an
    /// EFI_RT_PROPERTIES_TABLE configuration table
    pub fn get_time(&self) -> Result<TimeStruct, IgnixError> {
        let Some(function) = self.get_method() else {
            Err(Status::RST_POINTER_MISSING.context("get_time"))?
        };
        /* SAFETY
         * This unsafe doesn't do anything wrong, just need to use it because Rust is going to fill
         * every field of the struct with 0x00 and will let the EFI API fill it by itself.*/
        let mut time: Time = unsafe { zeroed() };
        let mut time_capabilities: TimeCapabilities = unsafe { zeroed() };
        let status = unsafe { (function.get_time)(&mut time, &mut time_capabilities) };
        if status.is_error() {
            Err(status.context("get_time"))?
        }
        Ok(TimeStruct {
            time,
            time_capabilities,
        })
    }
    /// The SetTime() function sets the real time clock device to the supplied time, and records
    /// the current time zone and daylight savings time information. The SetTime() function is not
    /// allowed to loop based on the current time. For example, if the device does not support a
    /// hw reset for the sub-resolution time, the code is not to implement the feature by
    /// waiting for the time to wrap.
    ///
    /// Warning from the UEFI spec:
    /// During runtime, if a PC-AT CMOS device is present in the platform the caller must
    /// synchronize access to the device before calling SetTime().
    ///
    /// RETURN CODES:
    /// EFI_INVALID_PARAMETER A time field is out of range.
    /// EFI_DEVICE_ERROR The time could not be set due to a hardware error.
    /// EFI_UNSUPPORTED This call is not supported by this platform at the time the call is made.
    /// The platform should describe this runtime service as unsupported at runtime via an
    /// EFI_RT_PROPERTIES_TABLE configuration table.
    pub fn set_time(&self, time: &Time) -> Result<(), IgnixError> {
        let Some(function) = self.get_method() else {
            Err(Status::RST_POINTER_MISSING.context("set_time"))?
        };
        let status = unsafe { (function.set_time)(time) };
        if status.is_error() {
            Err(status.context("set_time"))?
        }
        Ok(())
    }
    /// Returns the current wakeup alarm clock thingie
    ///
    /// The alarm clock time may be rounded from the set alarm clock time to be within the
    /// resolution of the alarm clock device. The resolution of the alarm clock device is defined
    /// to be one second.
    ///
    /// Warning from the UEFI spec:
    /// During runtime, if a PC-AT CMOS device is present in the platform the
    /// caller must synchronize access to the device before calling GetWakeupTime()
    ///
    /// RETURN CODES:
    /// EFI_INVALID_PARAMETER Enabled is NULL.
    /// EFI_INVALID_PARAMETER Pending is NULL.
    /// EFI_INVALID_PARAMETER Time is NULL.
    /// EFI_DEVICE_ERROR The wakeup time could not be retrieved due to a hardware error.
    /// EFI_UNSUPPORTED This call is not supported by this platform at the time the call is made.
    /// The platform should describe this runtime service as unsupported at runtime via an
    /// EFI_RT_PROPERTIES_TABLE configuration table.
    pub fn get_wakeup_time(&self) -> Result<WakeupTime, IgnixError> {
        let Some(function) = self.get_method() else {
            Err(Status::RST_POINTER_MISSING.context("get_wakeup_time"))?
        };
        let mut enabled: Boolean = Boolean(0);
        let mut pending: Boolean = Boolean(0);
        let mut time: Time = unsafe { zeroed() };
        let status = unsafe { (function.get_wakeup_time)(&mut enabled, &mut pending, &mut time) };
        if status.is_error() {
            Err(status.context("get_wakeup_time"))?
        }
        Ok(WakeupTime {
            enabled,
            pending,
            time,
        })
    }
    /// Sets the system wakeup alarm system
    ///
    /// Time: If enable is true, then time cannot be None. If enable is false, then it doesn't matter
    ///
    /// RETURN CODES:
    /// EFI_INVALID_PARAMETER A time field is out of range.
    /// EFI_DEVICE_ERROR The wakeup time could not be set due to a hardware error.
    /// EFI_UNSUPPORTED This call is not supported by this platform at the time the call is made.
    /// The platform should describe this runtime service as unsupported at runtime via an
    /// EFI_RT_PROPERTIES_TABLE configuration table.
    pub fn set_wakeup_time(&self, enable: bool, time: Option<&Time>) -> Result<(), IgnixError> {
        assert!(
            enable == true && time.is_none(),
            "Please, provide the time parameter (not none) in 'set_wakeup_timer' when enable == true"
        );
        let Some(function) = self.get_method() else {
            Err(Status::RST_POINTER_MISSING.context("set_wakeup_time"))?
        };
        let time_ptr = match time {
            None => core::ptr::null(),
            Some(ptr) => ptr,
        };
        let status = unsafe { (function.set_wakeup_time)(enable, time_ptr as *const Time) };
        if status.is_error() {
            Err(status.context("set_wakeup_time"))?
        }
        Ok(())
    }
}
