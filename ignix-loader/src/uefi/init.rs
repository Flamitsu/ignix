/*
 * Copyright (C) 2026 Flamitsu
 *
 * This file is part of Ignix.
 *
 * Ignix is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * Ignix is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Ignix.  If not, see <https://www.gnu.org/licenses/>.
 */
use crate::{SystemTable, uefi::types::Status};

static mut SYSTEM_TABLE_PTR: *const SystemTable = core::ptr::null();

pub fn init_services(system_table: *mut SystemTable) -> Result<(), Status> {
    unsafe { SYSTEM_TABLE_PTR = system_table }
    if system_table.is_null() {
        Err(Status::INVALID_PARAMETER)?
    }
    Ok(())
}

pub fn get_system_table() -> &'static SystemTable {
    unsafe {
        if SYSTEM_TABLE_PTR.is_null() {}
        &*SYSTEM_TABLE_PTR
    }
}
