// SPDX-License-Identifier: GPL-3.0-only
use crate::table::boot::BootServicesWrapper;
impl BootServicesWrapper {
    pub fn load_image(&self) {}
    pub fn start_image(&self) {}
    pub fn unload_image(&self) {}
    pub fn exit(&self) {}
    pub fn exit_boot_services(&self) {}
}
