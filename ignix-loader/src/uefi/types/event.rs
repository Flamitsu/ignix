// SPDX-License-Identifier: GPL-3.0-only
use core::ffi::c_void;

pub type Event = *mut c_void;
#[repr(transparent)]
pub struct EventType(pub u32);

impl EventType {
    /// The event is a timer event that may be passed to
    /// set_timer(). Note that timers only function during
    /// boot services time
    pub const EVT_TIMER: Self = Self(1 << 31);
    /// The event is allocated from runtime memory. If an event
    /// is to be signaled after the call to exit_boot_services() the event's
    /// data structure and notification function need to be allocated from
    /// runtime memory. For more information see set_virtual_address_map()
    pub const EVT_RUNTIME: Self = Self(1 << 30);

    /// If an event of this type is not already in the signaled state,
    /// then the event's NotificationFunction will be queued at the event's
    /// NotifyTpl whenever the event is being waited on via wait_for_event() or
    /// check_event()
    pub const EVT_NOTIFY_WAIT: Self = Self(1 << 8);
    /// The event's NotifyFunction is queued whenever the event is signaled
    pub const EVT_NOTIFY_SIGNAL: Self = Self(1 << 9);

    /// The event is of type EVT_NOTIFY_SIGNAL. It should not be combined with
    /// any other event types. This event type is functionally equivalent to
    /// EFI_EVENT_GROUP_EXIT_BOOT_SERVICES event group. Refer to that event group
    /// description in create_event_ex section below
    pub const EVT_SIGNAL_EXIT_BOOT_SERVICES: Self = Self(1 << 9 | 1);
    /// The event is to be notified by the system when set_virtual_address_map() is performed.
    /// This event type is a composite of EVT_NOTIFY_SIGNAL, EVT_RUNTIME and EVT_RUNTIME_CONTEXT
    /// and should not be combined with the other types.
    pub const EVT_SIGNAL_VIRTUAL_ADDRESS_CHANGE: Self = Self((1 << 31 | 1 << 9) | (1 << 1));
}

/* This is some shit that you can't blame me for.
 * Just check UEFI spec 2.11 page 141, and you
 * will see what I mean */
pub type EventNotifyFn = unsafe extern "efiapi" fn(event: Event, context: *mut c_void);

#[derive(Clone, Copy)] // Just in case you need to use it inside a loop
#[repr(C)]
pub struct Tpl(pub usize);
// Those numbers can be found in UEFI spec 2.11 page 150 section "Related definitions"
#[allow(unused)]
impl Tpl {
    pub const TPL_APPLICATION: Self = Self(4);
    pub const TPL_CALLBACK: Self = Self(8);
    pub const TPL_NOTIFY: Self = Self(16);
    pub const TPL_HIGH_LEVEL: Self = Self(31);
}
