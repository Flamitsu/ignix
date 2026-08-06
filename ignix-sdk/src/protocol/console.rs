// SPDX-License-Identifier: GPL-3.0-only
use crate::types::{Boolean, Event, Guid, Handle, IgnixError, Status, Uuid};
use core::{ffi::c_void, ptr::NonNull};
#[repr(C)]
pub struct SimpleTextOutputMode {
    pub max_mode: u32,
    pub mode: u32,
    pub attribute: u32,
    pub cursor_col: u32,
    pub cursor_row: u32,
    pub cursor_visible: Boolean,
}

#[repr(C)]
#[allow(unused)]
pub struct SimpleTextOutputProtocol {
    pub reset: unsafe extern "efiapi" fn(this: *mut Self, extended: bool) -> Status,
    pub output_string: unsafe extern "efiapi" fn(this: *mut Self, string: *const u16) -> Status,
    pub test_string: unsafe extern "efiapi" fn(this: *mut Self, string: *const u16) -> Status,
    pub query_mode: unsafe extern "efiapi" fn(
        this: *mut Self,
        mode: usize,
        columns: *mut usize,
        rows: *mut usize,
    ) -> Status,
    pub set_mode: unsafe extern "efiapi" fn(this: *mut Self, mode: usize) -> Status,
    pub set_attribute: unsafe extern "efiapi" fn(this: *mut Self, attribute: usize) -> Status,
    pub clear_screen: unsafe extern "efiapi" fn(this: *mut Self) -> Status,
    pub set_cursor_position:
        unsafe extern "efiapi" fn(this: *mut Self, column: usize, rows: usize) -> Status,
    pub enable_cursor: unsafe extern "efiapi" fn(this: *mut Self, visible: bool) -> Status,
    pub mode: *mut SimpleTextOutputMode,
}

impl Uuid for SimpleTextOutputProtocol {
    const GUID: Guid = Guid::new(
        0x387477c2,
        0x69c7,
        0x11d2,
        [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
    );
}

// This is the wrapper that allows to use SimpleTextOutputProtocol withouth unsafe lines
pub struct SimpleTextOutputProtocolWrapper {
    protocol: NonNull<SimpleTextOutputProtocol>,
}

impl SimpleTextOutputProtocolWrapper {
    /// # Safety
    /// The protocol should point to a valid instance assigned by the UEFI, and not null
    pub unsafe fn new(protocol: *mut SimpleTextOutputProtocol) -> Self {
        let not_null =
            NonNull::new(protocol).expect("SimpleTextOutputProtocol pointer cannot be null");
        Self { protocol: not_null }
    }

    fn get_protocol(&self) -> &SimpleTextOutputProtocol {
        unsafe { self.protocol.as_ref() }
    }

    pub fn reset(&mut self, extended: bool) -> Result<(), IgnixError> {
        let status = unsafe { (self.get_protocol().reset)(self.protocol.as_ptr(), extended) };
        if status.is_error() {
            Err(status.context("SimpleTextOutputProtocol.reset"))?
        }
        Ok(())
    }

    pub fn output_string(&mut self, string: &[u16]) -> Result<(), IgnixError> {
        let status =
            unsafe { (self.get_protocol().output_string)(self.protocol.as_ptr(), string.as_ptr()) };
        if status.is_error() {
            Err(status.context("SimpleTextOutputProtocol.output_string"))?
        }
        Ok(())
    }

    pub fn test_string(&mut self, string: &[u16]) -> Result<(), IgnixError> {
        let status =
            unsafe { (self.get_protocol().test_string)(self.protocol.as_ptr(), string.as_ptr()) };
        if status.is_error() {
            Err(status.context("SimpleTextOutputProtocol.test_string"))?
        }
        Ok(())
    }

    pub fn query_mode(
        &mut self,
        mode: usize,
        columns: &mut usize,
        rows: &mut usize,
    ) -> Result<(), IgnixError> {
        let status = unsafe {
            (self.get_protocol().query_mode)(self.protocol.as_ptr(), mode, columns, rows)
        };
        if status.is_error() {
            Err(status.context("SimpleTextOutputProtocol.query_mode"))?
        }
        Ok(())
    }

    pub fn set_mode(&mut self, mode: usize) -> Result<(), IgnixError> {
        let status = unsafe { (self.get_protocol().set_mode)(self.protocol.as_ptr(), mode) };
        if status.is_error() {
            Err(status.context("SimpleTextOutputProtocol.set_mode"))?
        }
        Ok(())
    }

    pub fn set_attribute(&mut self, attribute: usize) -> Result<(), IgnixError> {
        let status =
            unsafe { (self.get_protocol().set_attribute)(self.protocol.as_ptr(), attribute) };
        if status.is_error() {
            Err(status.context("SimpleTextOutputProtocol.set_attribute"))?
        }
        Ok(())
    }

    pub fn clear_screen(&mut self) -> Result<(), IgnixError> {
        let status = unsafe { (self.get_protocol().clear_screen)(self.protocol.as_ptr()) };
        if status.is_error() {
            Err(status.context("SimpleTextOutputProtocol.clear_screen"))?
        }
        Ok(())
    }
    pub fn set_cursor_position(&mut self, column: usize, row: usize) -> Result<(), IgnixError> {
        let status = unsafe {
            ((self.get_protocol().set_cursor_position)(self.protocol.as_ptr(), column, row))
        };
        if status.is_error() {
            Err(status.context("SimpleTextOutputProtocol.set_cursor_position"))?
        }
        Ok(())
    }

    pub fn enable_cursor(&mut self, visible: bool) -> Result<(), IgnixError> {
        let status =
            unsafe { (self.get_protocol().enable_cursor)(self.protocol.as_ptr(), visible) };
        if status.is_error() {
            Err(status.context("SimpleTextOutputProtocol.enable_cursor"))?
        }
        Ok(())
    }
}

#[repr(C)]
/* This is the EXTENDED VERSION of the Input Protocol. Means motherboards before 2006 may not support
 * this, but anyways they also do whathever they want with the firmware so its not even granted on
 * any other */
pub struct SimpleTextInputProtocol {
    pub reset:
        unsafe extern "efiapi" fn(this: *mut SimpleTextInputProtocol, extended: bool) -> Status,
    pub read_key_stroke: unsafe extern "efiapi" fn(
        this: *mut SimpleTextInputProtocol,
        keydata: *mut KeyData,
    ) -> Status,
    pub wait_for_key: Event,
    pub set_state: unsafe extern "efiapi" fn(
        this: *mut SimpleTextInputProtocol,
        key_toggle_state: KeyToggleState,
    ) -> Status,
    pub register_key_notify: unsafe extern "efiapi" fn(
        this: *mut SimpleTextInputProtocol,
        key_data: *mut KeyData,
        key_notification_function: KeyNotifyFunction,
        notify_handle: *mut Handle,
    ) -> Status,
    pub unregister_key_notify: unsafe extern "efiapi" fn(
        this: *mut SimpleTextInputProtocol,
        notification_handle: *const Handle,
    ) -> Status,
}

impl Uuid for SimpleTextInputProtocol {
    const GUID: Guid = Guid::new(
        0xdd9e7534,
        0x7762,
        0x4698,
        [0x8c, 0x14, 0xf5, 0x85, 0x17, 0xa6, 0x25, 0xaa],
    );
}

pub struct SimpleTextInputProtocolWrapper {
    protocol: NonNull<SimpleTextInputProtocol>,
}

impl SimpleTextInputProtocolWrapper {
    /// # Safety
    /// So this function is safe to use because if you don't have this protocol mapped
    /// in your firmware you're screwd anyways so it gives a panic and reduces code smell
    /// (Need to apply this pattern for others) so it's completely secure to use
    pub unsafe fn new(protocol: *mut SimpleTextInputProtocol) -> Self {
        let non_null =
            NonNull::new(protocol).expect("SimpleTextInputProtocol pointer cannot be null");
        Self { protocol: non_null }
    }
    // Safety:
    // The previous assert in the new function must be ALWAYS PRESENT so this unsafe code doesn't
    // blow everything up.
    #[inline(always)]
    fn get_protocol(&self) -> &SimpleTextInputProtocol {
        unsafe { self.protocol.as_ref() }
    }
}
#[repr(C)]
pub struct KeyData {
    pub key_unicode: u16,
    pub key_state: KeyState,
}

#[repr(C)]
pub struct KeyState {
    pub key_shift_state: KeyShiftState,
    pub key_toggle_state: KeyToggleState,
}

#[repr(C)]
pub struct KeyShiftState(pub u32);

impl KeyShiftState {
    pub const SHIFT_STATE_VALID: Self = Self(0x80000000);
    pub const RIGHT_SHIFT_PRESSED: Self = Self(0x00000001);
    pub const LEFT_SHIFT_PRESSED: Self = Self(0x00000002);
    pub const RIGHT_CONTROL_PRESSED: Self = Self(0x00000004);
    pub const LEFT_CONTROL_PRESSED: Self = Self(0x00000008);
    pub const RIGHT_ALT_PRESSED: Self = Self(0x00000010);
    pub const LEFT_ALT_PRESSED: Self = Self(0x00000020);
    /* I don't know why would you want to use this but if you want, have in mind this is the
     * 'SUPER' key, widely and wrong known as 'WINDOWS KEY', need to include both of them to work
     * properly on any keyboard */
    pub const RIGHT_LOGO_PRESSED: Self = Self(0x00000040);
    pub const LEFT_LOGO_PRESSED: Self = Self(0x00000080);
    pub const MENU_KEY_PRESSED: Self = Self(0x00000100);
    pub const SYS_REQ_PRESSED: Self = Self(0x00000200);
    pub fn contains(&self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub fn has_flag(&self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }
}
#[repr(C)]
pub struct KeyToggleState(pub u8);
impl KeyToggleState {
    pub const TOGGLE_STATE_VALID: Self = Self(0x80);
    pub const KEY_STATE_EXPOSED: Self = Self(0x40);
    pub const SCROLL_LOCK_ACTIVE: Self = Self(0x01);
    pub const NUM_LOCK_ACTIVE: Self = Self(0x02);
    pub const CAPS_LOCK_ACTIVE: Self = Self(0x01);
    pub fn contains(&self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub fn has_flag(&self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }
}
#[repr(C)]
pub struct KeyNotifyFunction {
    key_data: *const KeyData,
}
