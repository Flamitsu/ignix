use core::ptr::NonNull;

use crate::types::{Event, Guid, Handle, Status, Uuid};

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
