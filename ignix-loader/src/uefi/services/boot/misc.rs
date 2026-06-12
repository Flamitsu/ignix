// SPDX-License-Identifier: GPL-3.0-only
use crate::uefi::{table::boot::BootServicesWrapper, types::Status};
use core::time::Duration;
impl BootServicesWrapper {
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
}
