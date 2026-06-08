// SPDX-License-Identifier: GPL-3.0-only
use crate::boot::disk::DiskScanner;
use crate::cli::args::HookHelp;
use crate::IgnixError;
use crate::errors::cmd;
use crate::utils::SystemInfo;
pub fn help_hooks(options: HookHelp) -> Result<String, IgnixError>{
    if options.get_machine_id && !options.get_esp_mountpoint{
        return Ok(SystemInfo::new()?.machine_id.to_string());
    }

    if options.get_esp_mountpoint && !options.get_machine_id {
        return Ok(DiskScanner::new(false, false).find_compatible_esp()?
            .mountpoint.display().to_string())
    }
    Err(cmd::Error::InvalidArgument("invalid hook help arguments".into()))?
}
