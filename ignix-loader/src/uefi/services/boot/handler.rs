use core::ffi::c_void;
/* If you're wondering why this is in boot services and not in protocol,
 * I'm wondering the damn same. UEFI says this is a boot service, and if i don't put them here
 * the byte alignment will be awful. So, sorry for that ^^'*/
#[repr(C)]
pub struct Handler{
    install_protocol_interface: *mut c_void,
    uninstall_protocol_interface: *mut c_void,
    reinstall_protocol_interface: *mut c_void,
    register_protocol_notify: *mut c_void,
    locate_handle: *mut c_void,
    handle_protocol: *mut c_void,
    locate_device_path: *mut c_void,
    open_protocol: *mut c_void,
    close_protocol: *mut c_void,
    open_protocol_information: *mut c_void,
    connect_controller: *mut c_void,
    disconnect_controller: *mut c_void,
    protocols_per_handle: *mut c_void,
    locate_handle_buffer: *mut c_void,
    locate_protocol: *mut c_void,
    install_multiple_protocol_interfaces: *mut c_void,
    uninstall_multiple_protocol_interfaces: *mut c_void,
}
