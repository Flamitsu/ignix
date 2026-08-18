use core::sync::atomic::{AtomicPtr, Ordering};
use crate::{table::SystemTable, types::{Status, Handle}}; 
pub struct InitGlobalPtr<T> {
    ptr: AtomicPtr<T>,
}
/**/
impl<T> InitGlobalPtr<T> {
    #[inline(always)]
    pub const fn empty() -> Self {
        Self {
            ptr: AtomicPtr::new(core::ptr::null_mut()),
        }
    }
    /* Just a big disclaimer, UEFI is single-threaded. I'm doing this so its safe for rust and
     * lets me use this withouth using unsafe keyword.*/
    #[inline(always)]
    pub fn set(&self, item: *const T) -> Result<(), Status> {
        let item_mut = item as *mut T;
        self.ptr
            .compare_exchange(
                core::ptr::null_mut(),
                item_mut,
                Ordering::SeqCst,
                Ordering::SeqCst,
            )
            .map(|_| ())
            .map_err(|_| Status::ST_POINTER_MISSING)
    }

    #[inline(always)]
    pub fn get(&self) -> &'static T {
        let p = self.ptr.load(Ordering::SeqCst);
        if p.is_null() {
            panic!("Cannot get pointer. Pointer is null (System table || Handle)");
        }
        unsafe { &*p }
    }
}

pub static SYSTEM_TABLE: InitGlobalPtr<SystemTable> = InitGlobalPtr::empty();
pub static HANDLE: InitGlobalPtr<Handle> = InitGlobalPtr::empty();
