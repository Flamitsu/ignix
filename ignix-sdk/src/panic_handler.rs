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
