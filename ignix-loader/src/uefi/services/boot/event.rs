use core::{ffi::c_void, time::Duration};

// SPDX-License-Identifier: GPL-3.0-only
use crate::uefi::{
    init::SYSTEM_TABLE,
    table::boot::BootServicesWrapper,
    types::{Event, EventGroup, EventNotifyFn, EventType, Status, TimerDelay, Tpl},
};
impl BootServicesWrapper {
    pub fn raise_tpl(&self, new_tpl: Tpl) -> Option<TplGuardian> {
        let Some(function) = self.get_method() else {
            return None;
        };
        return Some(TplGuardian {
            old_tlp: unsafe { (function.raise_tpl)(new_tpl) },
        });
    }

    pub fn restore_tpl(&self, old_tpl: Tpl) {
        let Some(function) = self.get_method() else {
            return;
        };
        unsafe { (function.restore_tpl)(old_tpl) }
    }

    pub fn create_event(
        &self,
        event_type: EventType,
        tpl: Tpl,
        notify_function: Option<EventNotifyFn>,
        notify_context: *mut c_void,
    ) -> Result<Event, Status> {
        let mut event: Event = core::ptr::null_mut();
        let Some(function) = self.get_method() else {
            Err(Status::NOT_FOUND)?
        };
        let status = unsafe {
            (function.create_event)(
                event_type,
                tpl,
                notify_function,
                notify_context,
                &mut event as *mut Event,
            )
        };
        if status.is_success() {
            return Ok(event);
        }
        Err(status)?
    }

    pub fn create_event_ex(
        &self,
        event_type: EventType,
        tpl: Tpl,
        notify_function: Option<EventNotifyFn>,
        notify_context: *const c_void,
        event_group: Option<EventGroup>,
    ) -> Result<Event, Status> {
        let mut event: Event = core::ptr::null_mut();
        let Some(function) = self.get_method() else {
            Err(Status::NOT_FOUND)?
        };
        let event_group_ptr = match &event_group {
            Some(group) => &event_group as *const Option<EventGroup>,
            None => core::ptr::null(),
        };
        let status = unsafe {(function.create_event_ex)(
            event_type,
            tpl,
            notify_function,
            notify_context,
            event_group_ptr,
            &mut event,
        )};
        if status.is_success() {
            return Ok(event);
        }
        Err(status)?
    }

    pub fn close_event(&self, event: Event) -> Status {
        let Some(function) = self.get_method() else { return Status::NOT_FOUND; };
        let status = unsafe {(function.close_event)(event)};
        status
    }

    pub fn signal_event(&self, event: Event) -> Status {
        let Some(function) = self.get_method() else { return Status::NOT_FOUND;};
        let status = unsafe { (function.signal_event)(event) };
        status
    }

    pub fn wait_for_event(&self, event: Event) -> Result<usize, Status> {
        let Some(function) = self.get_method() else { return Err(Status::NOT_FOUND); };
        let mut index = 0;
        let status = unsafe { (function.wait_for_event)(0, &event, &mut index)};
        if status.is_success() {
            return Ok(index);
        }
        Err(status)?
    }

    pub fn check_event(&self, event: Event) -> Status {
        let Some(function) = self.get_method() else { return Status::NOT_FOUND };
        let status = unsafe { (function.check_event)(event) };
        status
    }

    pub fn set_timer(&self, event: Event, timer_delay: TimerDelay, trigger_time: Duration) -> Status {
        let Some(function) = self.get_method() else { return Status::NOT_FOUND };
        let ns: u64 = trigger_time.as_nanos().try_into().unwrap();
        let status = unsafe { (function.set_timer)(event, timer_delay, ns) };
        status
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
        SYSTEM_TABLE
            .get()
            .unwrap()
            .get_boot_services()
            .unwrap()
            .restore_tpl(self.old_tlp);
    }
}
