// SPDX-License-Identifier: GPL-3.0-only
use crate::{
    table::runtime::RuntimeServicesWrapper,
    types::{IgnixError, ResetType, Status},
};

impl RuntimeServicesWrapper {
    /// This is just a copy from the GetNextMonotonicCount but for the runtime services
    pub fn get_next_high_monotonic_count(&self) -> Result<u32, IgnixError> {
        let Some(function) = self.get_method() else {
            Err(Status::RST_POINTER_MISSING.context("get_next_high_monotonic_count"))?
        };
        let mut number: u32 = 0;
        let status = unsafe { (function.get_next_high_monotonic_count)(&mut number) };
        if status.is_error() {
            Err(status.context("get_next_high_monotonic_count"))?
        }
        Ok(number)
    }
    /// Resets the entire platform. If the platform supports See ref:EFI_RESET_NOTIFICATION_PROTOCOL,
    /// then prior to completing the reset of the platform, all of the pending notifications must
    /// be called
    pub fn reset_system<const N: usize>(
        &self,
        reset_type: ResetType,
        reset_status: Status,
        reset_data: [u16; N],
    ) -> Result<(), IgnixError> {
        let Some(function) = self.get_method() else {
            Err(Status::RST_POINTER_MISSING.context("reset_system"))?
        };
        unsafe {
            (function.reset_system)(
                reset_type,
                reset_status,
                reset_data.len(),
                reset_data.as_ptr(),
            )
        };
        Ok(())
    }
}
