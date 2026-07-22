// SPDX-License-Identifier: GPL-3.0-only
#[macro_export]
macro_rules! str_utf16 {
    ($text:expr) => {{
        let mut buffer = [0u16; $text.len() + 1];
        let mut i = 0;

        for c in $text.encode_utf16() {
            buffer[i] = c;
            i += 1;
        }
        buffer[i] = 0;
        buffer
    }};
}
