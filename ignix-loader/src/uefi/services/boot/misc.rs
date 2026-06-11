// SPDX-License-Identifier: GPL-3.0-only
use crate::uefi::{table::boot::BootServicesWrapper, types::Status};
use core::time::Duration;
impl BootServicesWrapper {
    pub fn stall(&self, duration: Duration) -> Result<Status,Status> {
        if let Some(function) = self.get_method() {
            let microseconds = duration.as_micros() as usize;
            let status = unsafe { (function.stall)(microseconds) };
            
            if status.is_error(){
                return Err(status)
            }
            
            return Ok(status)
        }
        Err(Status::NOT_FOUND)
    }
}
