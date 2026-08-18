// SPDX-License-Identifier: GPL-3.0-only
use crate::{init::SYSTEM_TABLE, println, services::boot::event::{close_event, restore_tpl}, types::Guid};
use core::ffi::c_void;

pub type Event = *mut c_void;

#[repr(transparent)]
#[derive(PartialEq, Eq)]
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
#[repr(transparent)]
pub struct Tpl(pub usize);
// Those numbers can be found in UEFI spec 2.11 page 150 section "Related definitions"
#[allow(unused)]
impl Tpl {
    /// the lowest priority level
    pub const TPL_APPLICATION: Self = Self(4);
    /// An intermediate priority level
    pub const TPL_CALLBACK: Self = Self(8);
    /// The highest priority level
    pub const TPL_NOTIFY: Self = Self(16);
    /// Higher than TPL_NOTIFY (I know it doesn't make sense,
    /// I'm only redacting what the UEFI spec says). Don't use it for
    /// long periods of time since it may cause inestability
    pub const TPL_HIGH_LEVEL: Self = Self(31);
}

#[repr(transparent)]
pub struct EventGroup(pub Guid);

impl EventGroup {
    /// This event group is notified whenever the system calls to ExitBootservices(),
    /// after notifying EFI_EVENT_GROUP_BEFORE_EXIT_BOOT_SERVICES event group.
    /// event group is functionally equivalent to the EVT_SIGNAL_EXIT_BOOT_SERVICES
    /// flag for the Type argument of CreateEvent.
    /// Notification requirements:
    /// - The notification function isn't allowed to use any Memory Allocation services in
    ///   any way (either itself or calling another function that uses it internally)
    /// - The notification function must not depend on timer events
    pub const EFI_EVENT_GROUP_EXIT_BOOT_SERVICES: Self = Self(Guid::new(
        0x27abf055,
        0xb1b8,
        0x4c26,
        [0x80, 0x48, 0x74, 0x8f, 0x37, 0xba, 0xa2, 0xdf],
    ));

    /// This event group is notified by the system ExitBootServices() is invoked right
    /// before notifying EFI_EVENT_GROUP_EXIT_BOOT_SERVICES event group. The event
    /// presents the last opportunity to use firmware interfaces in the boot environment.
    /// The notification function for this event must not depend on any kind of delayed
    /// processing (processing that happens in a timer callback beyond the time span
    /// of the notification function)
    pub const EFI_EVENT_GROUP_BEFORE_EXIT_BOOT_SERVICES: Self = Self(Guid::new(
        0x8be0e274,
        0x3970,
        0x4b44,
        [0x80, 0xc5, 0x1a, 0xb9, 0x50, 0x2f, 0x3b, 0xfc],
    ));

    /// This event group is notified by the system when SetVirtualAddressMap() is called.
    /// Equivalent to EVT_SIGNAL_VIRTUAL_ADDRESS_CHANGE flag for the type argument of
    /// CreateEvent
    pub const EFI_EVENT_GROUP_VIRTUAL_ADDRESS_CHANGE: Self = Self(Guid::new(
        0x13fa7698,
        0xc831,
        0x49c7,
        [0x87, 0xea, 0x8f, 0x43, 0xfc, 0xc2, 0x51, 0x96],
    ));

    /// This event group is notified by the system when the memory map changes.
    /// Notification function should not use Memory Allocate Services at all.
    pub const EFI_EVENT_GROUP_MEMORY_MAP_CHANGE: Self = Self(Guid::new(
        0x78bee926,
        0x692f,
        0x48fd,
        [0x9e, 0xdb, 0x1, 0x42, 0x2e, 0xf0, 0xd7, 0xab],
    ));

    /// This event group is notified by the system right before notifying
    /// EFI_EVENT_GROUP_AFTER_READY_TO_BOOT event group when the Boot Manager is about to
    /// load and execute a boot option or a platform or OS recovery option.
    pub const EFI_EVENT_GROUP_READY_TO_BOOT: Self = Self(Guid::new(
        0x7ce88fb3,
        0x4bd7,
        0x4679,
        [0x87, 0xa8, 0xa8, 0xd8, 0xde, 0xe5, 0xd, 0x2b],
    ));

    /// This event group is notified by the system immediately after notifying
    /// EFI_EVENT_GROUP_READY_TO_BOOT event group when the Boot Manager is about to load
    /// and execute a boot option or a platform or OS recovery option.
    pub const EFI_EVENT_GROUP_AFTER_READY_TO_BOOT: Self = Self(Guid::new(
        0x3a2a00ad,
        0x98b9,
        0x4cdf,
        [0xa4, 0x78, 0x70, 0x27, 0x77, 0xf1, 0xc1, 0xb],
    ));

    /// This event group is notified by the system when ResetSystem() is invoked and the
    /// system is about to be reset. The event group is only notified prior
    /// to ExitBootServices() invocation.
    pub const EFI_EVENT_GROUP_RESET_SYSTEM: Self = Self(Guid::new(
        0x62da6a56,
        0x13fb,
        0x485a,
        [0xa8, 0xda, 0xa3, 0xdd, 0x79, 0x12, 0xcb, 0x6b],
    ));
}
#[repr(C)]
pub enum TimerDelay {
    // The event's timer setting is cancelled and no time trigger is set. TriggerTime is ignored
    Cancel = 0,
    /* The event is signaled periodically at TriggerTime intervals from the current time.
     * This is the only timer trigger Type for which the event timer does not need to be
     * reset for each notification. All other timer trigger types are “one shot.”
     */
    Periodic = 1,
    // The event is to be signaled in TriggerTime 100ns units.
    Relative = 2,
}
/*
 * This struct is extremely important.
 * Try to call create_event() and then drop the context variable for example... congrats, you just
 * corrupted the memory
 */
pub struct IgnixEvent<'a> {
    pub raw_event: Event,
    pub context_ptr: *mut c_void,
    pub _m: core::marker::PhantomData<&'a c_void>, // Never in my life I though I needed to use this
}

impl<'a> Drop for IgnixEvent<'a> {
    fn drop(&mut self) {
        let _ = close_event(self.raw_event);
    }
}

/* NOTE FROM THE UEFI SPEC:
 * If NewTPL is below the current TPL level, then the system behaviour is indeterminate.
 * Executing TPLs ABOVE TPL_APPLICATION for longer periods of time may also result
 * in unpredictable behaviour
 * ( I was wondering how to manage this, looked to uefi-rs code in uefi/src/boot.rs
 * and it shows this same solution. Thank you guys. )
 * Just to clarify, this next section is licensed as:
 * SDPX-License identifier: MIT OR Apache 2.0 */
pub struct TplGuardian {
    pub old_tlp: Tpl,
}

impl TplGuardian {
    #[must_use]
    pub const fn get_old_tpl(&self) -> Tpl {
        self.old_tlp
    }
}

impl Drop for TplGuardian {
    fn drop(&mut self) {
        restore_tpl(self.old_tlp);
    }
}
