// SPDX-License-Identifier: GPL-3.0-only
use crate::{
    init::SYSTEM_TABLE,
    table::boot::BootServicesWrapper,
    types::{
        Event, EventGroup, EventNotifyFn, EventType, IgnixError, IgnixEvent, Status, TimerDelay,
        Tpl, TplGuardian,
    },
};
use core::{ffi::c_void, time::Duration};
impl BootServicesWrapper {
    /// Raises a task’s priority level and returns its previous level.
    pub fn raise_tpl(&self, new_tpl: Tpl) -> Option<TplGuardian> {
        let function = self.get_method()?;
        Some(TplGuardian {
            old_tlp: unsafe { (function.raise_tpl)(new_tpl) },
        })
    }
    /// Restores a task’s priority level to its previous value.
    /// Should not use this manually unless you know what you're doing.
    /// Since raise_tpl returns TplGuardian, that already restores the previous tpl
    /// calling this function internally on 'Drop'.
    pub fn restore_tpl(&self, old_tpl: Tpl) {
        let Some(function) = self.get_method() else {
            return;
        };
        unsafe { (function.restore_tpl)(old_tpl) }
    }
    /// Creates an event
    /// RETURN CODES:
    ///
    /// EFI_INVALID_PARAMETER One of the parameters has an invalid value.
    /// EFI_INVALID_PARAMETER Event is NULL.
    /// EFI_INVALID_PARAMETER Type has an unsupported bit set.
    /// EFI_INVALID_PARAMETER Type has both EVT_NOTIFY_SIGNAL and EVT_NOTIFY_WAIT set.
    /// EFI_INVALID_PARAMETER Type has either EVT_NOTIFY_SIGNAL or EVT_NOTIFY_WAIT set and
    /// NotifyFunction is NULL.
    /// EFI_INVALID_PARAMETER Type has either EVT_NOTIFY_SIGNAL or EVT_NOTIFY_WAIT set and
    /// NotifyTpl is not a supported TPL level.
    /// EFI_OUT_OF_RESOURCES The event could not be allocated
    pub fn create_event<'a, T>(
        &self,
        event_type: EventType,
        tpl: Tpl,
        notify_function: Option<EventNotifyFn>,
        data_context: &'a mut T,
    ) -> Result<IgnixEvent<'a>, IgnixError> {
        let mut event: Event = core::ptr::null_mut();
        let Some(function) = self.get_method() else {
            Err(Status::BST_POINTER_MISSING.context("create_event"))?
        };
        let notify_context = data_context as *mut T as *mut c_void;
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
            return Ok(IgnixEvent {
                context_ptr: notify_context,
                raw_event: event,
                _m: core::marker::PhantomData,
            });
        }
        Err(status.context("create_event"))?
    }

    /// Creates an event group
    /// When you signal one event group, all the others events of the group are also
    /// signaled. They execute in order of their TPL. One event is only capable of being in
    /// one group at a time, until you execute close_event() function that deletes it from the
    /// group.
    ///
    /// Warnings:
    /// - You can't use event types like EVT_SIGNAL_EXIT_BOOT_SERVICES
    ///   or EVT_SIGNAL_VIRTUAL_ADDRESS_CHANGE here. If you do, it will throw at you an error.
    /// - You can't use event types like EVT_NOTIFY_SIGNAL or EVT_NOTIFY_WAIT with the parameter
    ///   notify_function in None. It will give you an INVALID_PARAMETER status
    ///
    /// RETURN CODES:
    ///
    ///  EFI_INVALID_PARAMETER One of the parameters has an invalid value.
    ///  EFI_INVALID_PARAMETER Event is NULL.
    ///  EFI_INVALID_PARAMETER Type has an unsupported bit set.
    ///  EFI_INVALID_PARAMETER Type has both EVT_NOTIFY_SIGNAL and EVT_NOTIFY_WAIT set.
    ///  EFI_INVALID_PARAMETER Type has either EVT_NOTIFY_SIGNAL or EVT_NOTIFY_WAIT set and
    ///  NotifyFunction is NULL.
    ///  EFI_INVALID_PARAMETER Type has either EVT_NOTIFY_SIGNAL or EVT_NOTIFY_WAIT set and NotifyTpl is not a supported TPL level.
    /// EFI_OUT_OF_RESOURCES The event could not be allocated.
    pub fn create_event_ex<'a, T>(
        &self,
        event_type: EventType,
        tpl: Tpl,
        notify_function: Option<EventNotifyFn>,
        notify_context: &'a T,
        event_group: Option<*const EventGroup>,
    ) -> Result<IgnixEvent<'a>, IgnixError> {
        let mut event: Event = core::ptr::null_mut();
        let Some(function) = self.get_method() else {
            Err(Status::BST_POINTER_MISSING.context("create_event_ex"))?
        };
        if event_type == EventType::EVT_SIGNAL_EXIT_BOOT_SERVICES
            || event_type == EventType::EVT_SIGNAL_VIRTUAL_ADDRESS_CHANGE
        {
            Err(Status::INVALID_PARAMETER.context("create_event_ex"))?
        }

        if (event_type == EventType::EVT_NOTIFY_SIGNAL || event_type == EventType::EVT_NOTIFY_WAIT)
            && notify_function.is_none()
        {
            Err(Status::INVALID_PARAMETER.context("create_event_ex"))?
        }

        let event_group_ptr = match event_group {
            Some(ptr) => ptr,
            None => core::ptr::null(),
        };
        let data_context = notify_context as *const T as *const c_void;
        let status = unsafe {
            (function.create_event_ex)(
                event_type,
                tpl,
                notify_function,
                data_context,
                event_group_ptr,
                &mut event,
            )
        };

        if status.is_success() {
            return Ok(IgnixEvent {
                raw_event: event,
                context_ptr: data_context as *mut c_void,
                _m: core::marker::PhantomData,
            });
        }

        Err(status.context("create_event_ex"))?
    }

    /// Close an event
    /// This function is called when IgnixEvent drops off memory automatically.
    /// You should not use this function manually basically.
    /// If Event was registered with RegisterProtocolNotify() then CloseEvent() will remove the
    /// corresponding registration. It is safe to call CloseEvent() within the corresponding notify
    /// function.
    pub fn close_event(&self, event: Event) -> Result<(), IgnixError> {
        let Some(function) = self.get_method() else {
            return Err(Status::BST_POINTER_MISSING.context("close_event"));
        };

        let status = unsafe { (function.close_event)(event) };
        if status.is_error() {
            Err(status.context("close_event"))?
        }
        Ok(())
    }
    /// Signals an event
    /// If Event is of type EVT_NOTIFY_SIGNAL, then the event’s notification function is
    /// scheduled to be invoked at the event’s notification task priority level. SignalEvent()
    /// may be invoked from any task priority level. If the supplied Event is a part of an event
    /// group, then all of the events in the event group are also signaled and their notification
    /// functions are scheduled.
    pub fn signal_event(&self, event: &IgnixEvent) -> Result<(), IgnixError> {
        let Some(function) = self.get_method() else {
            return Err(Status::BST_POINTER_MISSING.context("signal_event"));
        };

        let status = unsafe { (function.signal_event)(event.raw_event) };
        if status.is_error() {
            Err(status.context("signal_event"))?
        }
        Ok(())
    }
    /// Stop the execution until an event is signaled.
    /// This function must be called at TPL_APPLICATION.
    /// Index parameter always represents the event
    /// if an event is of type EVT_NOTIFY_SIGNAL, then EFI_INVALID_PARAMETER is returned
    /// if an event is in the signaled state, the signaled state is cleared if an event is not in
    /// the signaled state but does have a notification function, the notification function is
    /// queued at the event’s notification task priority level.
    /// If the execution of the event’s notification function causes the event to be signaled,
    /// then the signaled state is cleared
    ///
    /// To wait for a specified time, a timer event must be included in the Event array
    ///
    /// RETURN CODES:
    /// EFI_INVALID_PARAMETER NumberOfEvents is 0.
    /// EFI_INVALID_PARAMETER The event indicated by Index is of type EVT_NOTIFY_SIGNAL.
    /// EFI_UNSUPPORTED The current TPL is not TPL_APPLICATION.
    pub fn wait_for_event<const N: usize>(&self, event: &[Event]) -> Result<usize, IgnixError> {
        let Some(function) = self.get_method() else {
            return Err(Status::BST_POINTER_MISSING.context("wait_for_event"));
        };
        let mut index = 0;
        let status = unsafe { (function.wait_for_event)(event.len(), event.as_ptr(), &mut index) };
        if status.is_success() {
            return Ok(index);
        }
        Err(status.context("wait_for_event"))
    }
    /// Checks whether an event is in a signaled state.
    /// RETURN CODES:
    /// EFI_NOT_READY The event is not in the signaled state.
    /// EFI_INVALID_PARAMETER Event is of type EVT_NOTIFY_SIGNAL.
    pub fn check_event(&self, event: Event) -> Result<(), IgnixError> {
        let Some(function) = self.get_method() else {
            return Err(Status::BST_POINTER_MISSING.context("check_event"));
        };

        let status = unsafe { (function.check_event)(event) };
        if status.is_error() {
            Err(status.context("check_event"))?
        }
        Ok(())
    }
    /// Sets the type of timer and the trigger time for a timer event.
    /// The SetTimer() function cancels any previous time trigger setting for the event, and sets
    /// the new trigger time for the event. This function can only be used on events of type
    /// EVT_TIMER.
    /// RETURN CODES:
    /// EFI_INVALID_PARAMETER Event or Type is not valid.
    pub fn set_timer(
        &self,
        event: Event,
        timer_delay: TimerDelay,
        trigger_time: Duration,
    ) -> Result<(), IgnixError> {
        let Some(function) = self.get_method() else {
            Err(Status::BST_POINTER_MISSING.context("set_timer"))?
        };

        let status = unsafe {
            (function.set_timer)(
                event,
                timer_delay,
                trigger_time.as_nanos().try_into().unwrap(),
            )
        };
        if status.is_error() {
            Err(status.context("set_timer"))?
        }
        Ok(())
    }
}
