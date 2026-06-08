// SPDX-License-Identifier: GPL-3.0-only
use crate::uefi::{table::boot::BootServicesWrapper, types::{Status}};
use core::time::Duration;
impl BootServicesWrapper {
    pub fn stall(&self, duration: Duration) -> Status {
        
        if let Some(function) = self.get_method() {
            let microseconds = duration.as_micros() as usize;
            return unsafe {(function.stall)(2_000_000)};
        }

        Status::INVALID_PARAMETER
    }
}
