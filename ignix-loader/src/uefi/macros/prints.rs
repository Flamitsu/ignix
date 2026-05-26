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
use core::fmt::{self, Write};

use crate::uefi::init::SYSTEM_TABLE;
pub struct Writer;
impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let st = SYSTEM_TABLE.get().unwrap();

        let con_out = st.con_out;
        let mut buffer = [0u16; 128];
        let mut i = 0;
        for c in s.encode_utf16() {
            if i < buffer.len() - 1 {
                buffer[i] = c;
                i += 1;
            } else {
                buffer[i] = 0;
                unsafe {
                    ((*con_out).output_string)(con_out, buffer.as_ptr());
                }
                buffer[0] = c;
                i = 1;
            }
        }
        if i > 0{
            buffer[i] = 0;
            unsafe {
                (((*con_out).output_string)(con_out, buffer.as_ptr()));
            }
        }
        Ok(())
    }
}
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let mut writer = $crate::uefi::macros::prints::Writer;
        let _ = core::write!(writer, $($arg)*);
    }};
}
/// Macro to print in each line. Max buffer default is 128 characters
#[macro_export]
macro_rules! println {
    // If not it will be just a normal print with a return carriage and next line
    () => {
        $crate::print!("\r\n");
    };
    // This is a pattern that catchs parameters like {int} for example to show them into a print
    ($($arg:tt)*) => {
        $crate::print!("{}\r\n", format_args!($($arg)*));
    };
}
