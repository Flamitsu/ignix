use core::time;

// SPDX-License-Identifier: GPL-3.0-only
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
    pub const TIMEOUT: &'static str = "timeout ";
}
