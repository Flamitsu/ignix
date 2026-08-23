use crate::config::{CONFIG_ROUTE, ConfigKeywords, LoaderConfig, LoaderData};
use core::{ffi::c_void, marker::PhantomData, mem::forget, ptr::null_mut};
use ignix_sdk::{
    init::{HANDLE, INITRD_FILES},
    protocol::{
        DevicePathNode, DevicePathProtocol, VendorDevicePathNode,
        loaded_image::LoadedImageProtocol,
        media::{
            File, FileAttributes, LINUX_EFI_INITRD_MEDIA_GUID, LoadFile2, OpenModes,
            SimpleFileSystem, SimpleFileSystemFFI,
        },
    },
    services::boot::{
        handler::{install_protocol_interface, open_protocol},
        image::{load_image, start_image},
        memory::allocate_pool,
    },
    str_utf16,
    types::{IgnixError, IgnixImage, InterfaceType, MemoryType, OpenProtocolAttributes, Uuid},
};

pub fn read_config(fs: &mut File) -> Result<LoaderConfig, IgnixError> {
    let mut timeout: usize = 0;

    let file_name = str_utf16!(CONFIG_ROUTE);
    let mut conf_file = fs.open(&file_name, OpenModes::READ, FileAttributes::NONE)?;
    let mut buffer = [0u8; 1024];
    let valid_bytes = conf_file.read(&mut buffer)?;
    let mut bytes = &buffer[..valid_bytes];

    while !bytes.is_empty() {
        if bytes[0] == b'#' {
            while !bytes.is_empty() && bytes[0] != b'\n' {
                bytes = &bytes[1..];
            }

            if !bytes.is_empty() {
                bytes = &bytes[1..]; // This is basically so \n is skipped
            }

            continue;
        }

        if bytes.starts_with(ConfigKeywords::TIMEOUT) {
            bytes = &bytes[ConfigKeywords::TIMEOUT.len()..];
            if !bytes.is_empty() && bytes[0] == b' ' {
                bytes = &bytes[1..];
            }

            let mut parsed_value: usize = 0;
            let mut found_digit = false;
            while !bytes.is_empty() && bytes[0].is_ascii_digit() {
                let digit = (bytes[0] - b'0') as usize;
                if let Some(number) = parsed_value
                    .checked_mul(10)
                    .and_then(|v| v.checked_add(digit))
                {
                    parsed_value = number;
                    found_digit = true;
                }
                bytes = &bytes[1..];
            }
            if found_digit {
                timeout = parsed_value;
            }
            continue;
        }
        bytes = &bytes[1..]
    }
    Ok(LoaderConfig { timeout })
}
#[allow(unused)]
pub fn load_entries<'a>(// fs: &mut FileProtocolWrapper
) -> Result<LoaderData<'a>, IgnixError> {
    // let entries = fs.open(&str_utf16!(ENTRIES_DIR), OpenModes::READ, FileAttributes::DIRECTORY)?;
    Ok(LoaderData::new())
}

pub fn load_kernel(kernel_name: &[u16]) -> Result<(), IgnixError> {
    let mut fs = open_root_fs()?;
    let mut file = fs.open(kernel_name, OpenModes::READ, FileAttributes::NONE)?;
    let file_size: usize = file.get_info()?.file_size.try_into().unwrap();
    let mut source_buffer = allocate_pool(MemoryType::EfiLoaderData, file_size)?;
    file.read(source_buffer.as_mut_slice(file_size))?;

    let kernel_handle = load_image(false, None, Some(source_buffer.as_mut_slice(file_size)))?;
    let mut loaded_kernel = open_protocol::<LoadedImageProtocol>(
        &kernel_handle.handle.unwrap(),
        &LoadedImageProtocol::GUID,
        OpenProtocolAttributes::GET_PROTOCOL,
    )?;

    let cmdline = &str_utf16!("root=UUID=78b80ce8-f663-4e11-9b96-d036a4d0082d rw");
    loaded_kernel.set_load_options(cmdline);
    start_image(kernel_handle).map_err(|(err, _image)| err)?;
    Ok(())
}

pub fn load_initrds(initrd_name: &[u16]) -> Result<(), IgnixError> {
    let mut fs = open_root_fs()?;
    let mut initrd_file = fs.open(initrd_name, OpenModes::READ, FileAttributes::NONE)?;
    let file_size: usize = initrd_file.get_info()?.file_size.try_into().unwrap();

    let mut source_buffer = allocate_pool(MemoryType::EfiLoaderData, file_size)?;
    initrd_file.read(source_buffer.as_mut_slice(file_size))?;
    INITRD_FILES.set(source_buffer);

    let mut initrd_image = IgnixImage {
        handle: Some(null_mut()),
        _m: PhantomData,
    };

    let device_path = DevicePathNode::new(
        0x04,
        0x03,
        VendorDevicePathNode {
            guid: LINUX_EFI_INITRD_MEDIA_GUID,
        },
    );

    install_protocol_interface(
        &mut initrd_image,
        &DevicePathProtocol::GUID,
        InterfaceType::Native,
        Some(&device_path as *const _ as *mut c_void),
    )?;

    install_protocol_interface(
        &mut initrd_image,
        &LoadFile2::GUID,
        InterfaceType::Native,
        Some(&INITRD_FILES.ffi as *const _ as *mut c_void),
    )?;

    // This is done so rust don't clean this memory (needed so the kernel can find this)
    forget(initrd_image);
    forget(device_path);
    Ok(())
}
pub fn open_root_fs() -> Result<File, IgnixError> {
    let image_guard = open_protocol::<LoadedImageProtocol>(
        &HANDLE.get(),
        &LoadedImageProtocol::GUID,
        OpenProtocolAttributes::GET_PROTOCOL,
    )?;
    let device_handle = image_guard.device_handle;
    let fs_guard = open_protocol::<SimpleFileSystemFFI>(
        &device_handle,
        &SimpleFileSystem::GUID,
        OpenProtocolAttributes::GET_PROTOCOL,
    )?;
    let mut sfsp = unsafe { SimpleFileSystem::new(fs_guard.interface) };
    sfsp.open_volume()
}
