use crate::config::{CONFIG_ROUTE, ConfigKeywords, LoaderConfig, LoaderData};
use ignix_sdk::{
    init::HANDLE,
    protocol::{
        media::{FileAttributes, File, OpenModes},
        loaded_image::LoadedImageProtocol,
        media::{SimpleFileSystem, SimpleFileSystemFFI},
    },
    services::boot::handler::open_protocol,
    str_utf16,
    types::{IgnixError, OpenProtocolAttributes, Uuid},
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

pub fn load_kernel() -> Result<(), IgnixError> {
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
