use crate::{
    protocol::{DevicePathNode, DevicePathProtocol, VendorDevicePathNode, media::{LINUX_EFI_INITRD_MEDIA_GUID, LoadFile2FFI, initrd_load_file}}, table::SystemTable, types::{Handle, PoolBuffer, Status},
};
use core::{
    ffi::c_void,
    ptr::null_mut,
    sync::atomic::{AtomicPtr, AtomicUsize, Ordering},
};
pub struct InitGlobalSystemTable {
    ptr: AtomicPtr<SystemTable>,
}
impl InitGlobalSystemTable {
    #[inline(always)]
    pub const fn empty() -> Self {
        Self {
            ptr: AtomicPtr::new(null_mut()),
        }
    }
    /* Just a big disclaimer, UEFI is single-threaded. I'm doing this so its safe for rust and
     * lets me use this withouth using unsafe keyword.*/
    #[inline(always)]
    pub fn set(&self, item: *const SystemTable) -> Result<(), Status> {
        let item_mut = item as *mut SystemTable;
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
    pub fn get(&self) -> &'static SystemTable {
        let p = self.ptr.load(Ordering::SeqCst);
        if p.is_null() {
            panic!("Cannot get System table pointer. Pointer is null");
        }
        unsafe { &*p }
    }
}

pub struct InitGlobalHandle {
    ptr: AtomicPtr<c_void>,
}

impl InitGlobalHandle {
    #[inline(always)]
    pub const fn empty() -> Self {
        Self {
            ptr: AtomicPtr::new(null_mut()),
        }
    }

    #[inline(always)]
    pub fn set(&self, handle: Handle) -> Result<(), Status> {
        self.ptr
            .compare_exchange(null_mut(), handle, Ordering::SeqCst, Ordering::SeqCst)
            .map(|_| ())
            .map_err(|_| Status::NOT_FOUND)
    }

    #[inline(always)]
    pub fn get(&self) -> Handle {
        let p = self.ptr.load(Ordering::SeqCst);
        if p.is_null() {
            panic!("Cannot get Handle. Pointer is null");
        }
        p as Handle
    }
}

#[repr(C)]
pub struct InitrdManager {
    ptr: AtomicPtr<u8>,
    len: AtomicUsize,
    pub ffi: LoadFile2FFI,
    initrd_device_path: DevicePathNode<VendorDevicePathNode>
}

impl InitrdManager {
    pub const fn new() -> Self {
        let node_size = (size_of::<DevicePathProtocol>() + size_of::<VendorDevicePathNode>()) as u16;
        Self {
            ptr: AtomicPtr::new(null_mut()),
            len: AtomicUsize::new(0),
            ffi: LoadFile2FFI {
                load_file: initrd_load_file,
            },
            initrd_device_path: DevicePathNode::<VendorDevicePathNode> {
                hdr: DevicePathProtocol {
                    r#type: 0x04,
                    subtype: 0x03,
                    length: node_size.to_le_bytes()
                },
                node: VendorDevicePathNode {
                    guid: LINUX_EFI_INITRD_MEDIA_GUID
                },
                end: DevicePathProtocol {
                    r#type: 0x7F,
                    subtype: 0xFF,
                    length: [4,0]
                }
            }
        }
    }
    
    pub fn get_linux_path_ptr(&self) -> *const DevicePathNode<VendorDevicePathNode> {
        core::ptr::addr_of!(self.initrd_device_path)
    }

    pub fn set(&self, buffer: PoolBuffer) {
        self.ptr.store(buffer.ptr.as_ptr(), Ordering::SeqCst);
        self.len.store(buffer.num_bytes, Ordering::SeqCst);
        core::mem::forget(buffer);
    }

    pub fn len(&self) -> usize {
        self.len.load(Ordering::SeqCst)
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.ptr.load(Ordering::SeqCst)
    }
}
pub static INITRD_MANAGER: InitrdManager = InitrdManager::new();
pub static SYSTEM_TABLE: InitGlobalSystemTable = InitGlobalSystemTable::empty();
pub static HANDLE: InitGlobalHandle = InitGlobalHandle::empty();
