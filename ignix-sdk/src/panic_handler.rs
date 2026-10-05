// SPDX-License-Identifier: GPL-3.0-only
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use crate::println;
    println!("{}", info);
    loop {
        unsafe {
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            core::arch::asm!("hlt");
        }
    }
}
