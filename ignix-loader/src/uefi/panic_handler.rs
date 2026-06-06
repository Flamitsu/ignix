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
#[cfg(not(test))]
#[panic_handler]
#[allow(unused)]

fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            core::arch::asm!("hlt");
            /* Those archs uses the same instruction 'wfi' to
             * put the CPU into a deep C state*/
            #[cfg(any(
                target_arch = "arm",
                target_arch = "aarch64",
                target_arch = "riscv64",
                target_arch = "riscv32"
            ))]
            core::arch::asm!("wfi");
        }
    }
}
