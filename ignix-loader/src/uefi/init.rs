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
use crate::uefi::table::SystemTable;

pub struct InitSystemTable {
    ptr: core::cell::UnsafeCell<*const SystemTable>,
}

unsafe impl Sync for InitSystemTable {}

impl InitSystemTable {
    pub const fn empty() -> Self {
        Self {
            ptr: core::cell::UnsafeCell::new(core::ptr::null()),
        }
    }

    pub fn set(&self, table: *const SystemTable) -> Result<(), ()> {
        unsafe {
            let current = *self.ptr.get();
            if !current.is_null() {
                return Err(());
            }
            *self.ptr.get() = table;
            Ok(())
        }
    }

    pub fn get(&self) -> Option<&'static SystemTable> {
        unsafe {
            let p = *self.ptr.get();
            if p.is_null() {
                None
            } else {
                Some(&*p)
            }
        }
    }
}

pub static SYSTEM_TABLE: InitSystemTable = InitSystemTable::empty();
