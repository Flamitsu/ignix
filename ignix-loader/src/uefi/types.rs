use core::ffi::c_void;

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
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Ignix.  If not, see <https://www.gnu.org/licenses/>.
 */
#[repr(transparent)]
pub struct Status(pub usize);
impl Status {
    pub const SUCCESS: Self = Status(0);
}
#[allow(unused)]
pub struct SimpleTextOutputProtocol{
    reset: *mut c_void,
    pub output_string: unsafe extern "efiapi" fn(this: *mut SimpleTextOutputProtocol,
        string: *const u16)
}
