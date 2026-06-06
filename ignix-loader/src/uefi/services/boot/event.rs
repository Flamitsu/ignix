use core::ffi::c_void;

#[repr(C)]
pub struct Event{
    pub create_event: *mut c_void,
    pub create_event_ex: *mut c_void,
    pub close_event: *mut c_void,
    pub signal_event: *mut c_void,
    pub wait_for_event: *mut c_void,
    pub check_event: *mut c_void,
    pub set_timer: *mut c_void,
    pub raise_tpl: *mut c_void,
    pub restore_tpl: *mut c_void,
}
