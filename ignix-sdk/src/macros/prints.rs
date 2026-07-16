// SPDX-License-Identifier: GPL-3.0-only
use core::fmt::{self, Write};

use crate::init::SYSTEM_TABLE;
pub struct Writer;
impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let st = SYSTEM_TABLE.get().unwrap();

        let mut con_out = st.get_stdout().unwrap();
        let mut buffer = [0u16; 128];
        let mut i = 0;

        for c in s.encode_utf16() {
            if i < buffer.len() - 1 {
                buffer[i] = c;
                i += 1;
            } else {
                buffer[i] = 0;
                con_out.output_string(&buffer);
                buffer[0] = c;
                i = 1;
            }
        }

        if i > 0 {
            buffer[i] = 0;
            con_out.output_string(&buffer);
        }
        Ok(())
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
