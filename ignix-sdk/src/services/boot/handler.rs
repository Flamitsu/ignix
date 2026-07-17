// SPDX-License-Identifier: GPL-3.0-only
use crate::table::boot::BootServicesWrapper;
impl BootServicesWrapper {
    pub fn install_protocol_interface(&self) {}
    pub fn uninstall_protocol_interface(&self) {}
    pub fn reinstall_protocol_interface(&self) {}
    pub fn register_protocol_notify(&self) {}
    pub fn locate_handle(&self) {}
    pub fn handle_protocol(&self) {}
    pub fn locate_device_path(&self) {}
    pub fn open_protocol(&self) {}
    pub fn close_protocol(self) {}
    pub fn open_protocol_information(&self) {}
    pub fn connect_controller(&self) {}
    pub fn disconnect_controller(&self) {}
    pub fn protocols_per_handle(&self) {}
    pub fn locate_handle_buffer(&self) {}
    pub fn locate_protocol(&self) {}
}
