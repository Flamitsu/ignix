// SPDX-License-Identifier: GPL-3.0-only
use core::{ffi::c_void, ptr::null_mut};
pub const CONFIG_ROUTE: &str = "\\loader\\ignix\\loader.conf";
pub const ENTRIES_DIR: &str = "\\loader\\entries";

pub struct LoaderConfig {
    pub timeout: usize,
}

impl LoaderConfig {
    pub fn new() -> Self {
        Self { timeout: 0 }
    }
}

pub struct ConfigKeywords {}
impl ConfigKeywords {
    pub const TIMEOUT: &[u8; 7] = b"timeout";
}

// Step time in which the polling loop completes a loop
pub const STEP_MS: u8 = 100;

pub struct LoaderData<'a> {
    pub entries: &'a [u8],
    pub num_entries: usize,
}
impl<'a> LoaderData<'a> {
    pub fn new() -> Self {
        Self { entries: &[0u8;1], num_entries: 1}
    }
}

pub struct Loader<'a> {
    loader_config: LoaderConfig,
    loader_data: LoaderData<'a>,
}
