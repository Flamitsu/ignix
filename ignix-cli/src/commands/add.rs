// SPDX-License-Identifier: GPL-3.0-only
use crate::boot::disk::DiskScanner;
use crate::cli::args::AddOptions;
use crate::errors::IgnixError;
use std::fs::{self, File};

pub fn add_entry(options: AddOptions) -> Result<(), IgnixError> {
    let disk = DiskScanner::new(false, false);
    let esp = disk.find_compatible_esp()?;
    let entries_route = esp.mountpoint.join("loader/entries");

    if !entries_route.exists() {
        fs::create_dir_all(&entries_route)?;
    }

    let file_name = format!("{}-{}.conf", options.machine_id, options.kernel_version);
    let file_path = entries_route.join(&file_name);
    let tmp_path = entries_route.join(format!("{}.tmp", file_name));
    let mut file_content = format!(
        "title       {}\n\
         version     {}\n\
         machine-id  {}\n\
         sort-key    {}\n\
         options     {}\n\
         linux       {}\n",
        options.title.trim_matches('\''),
        options.kernel_version,
        options.machine_id,
        options.sort_key.trim_matches('\''),
        options.options,
        options.linux
    );

    for initrd in options.initrd {
        file_content.push_str(&format!("initrd      {}\n", initrd));
    }

    fs::write(&tmp_path, &file_content)?;
    fs::rename(&tmp_path, &file_path)?;

    let dir = File::open(&entries_route)?;
    dir.sync_all()?;
    Ok(())
}
