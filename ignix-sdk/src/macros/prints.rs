// SPDX-License-Identifier: GPL-3.0-only
use core::fmt::{self, Write};
use crate::init::SYSTEM_TABLE;

const BUFF_LEN: usize = 128;

pub struct Writer;
impl Writer {
    fn flush(buffer: &mut [u16;BUFF_LEN], len: usize) -> fmt::Result {
        if len == 0 {
            return Ok(());
        }
        let mut con_out = SYSTEM_TABLE.get().get_stdout().unwrap(); 
        buffer[len] = 0;
        con_out.output_string(&buffer[..=len]).map_err(|_| fmt::Error)?;
        Ok(())
    }
}
impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut buffer = [0u16;BUFF_LEN];
        let mut i = 0;
        for c in s.encode_utf16() {
            if i >= BUFF_LEN - 1 {
                Self::flush(&mut buffer, i)?;
                i = 0;
            }
            buffer[i] = c;
            i += 1;
        }
        Self::flush(&mut buffer, i)
    }
}
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let mut writer = $crate::macros::prints::Writer;
        let _ = core::write!(writer, $($arg)*);
    }};
}

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
