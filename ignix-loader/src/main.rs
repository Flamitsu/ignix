#![no_std] // No standard library imported (Since no OS is running for now)
#![no_main] // No main function execution, needed the #[entry] point
mod input; // Input module imported (Just handles user input)
mod configuration; // Configuration module imported (Parse the config file)
mod find_boot; // Find the boot entries
mod kernel; // Used to load the entry selected
use uefi::prelude::*; // prelude methods imported
use uefi::println; // Prinln macro of the uefi crate

/*
* Meant to be the core of the project, work in progress. Need first to configure and install the
* general binary and then continue with the UEFI work
*/

// The entry point of the binary is needed so rust knows where the program starts. 
#[entry]
fn main() -> Status {
    // Start the UEFI services for the init system 
    uefi::helpers::init().unwrap();
    // Find the boot entries in the ESP partition
    find_boot::find_boot_entry();
    // Parse the config file to get the global configuration working 
    configuration::load_config(); 
    // Function that should show the boot entries of the ESP partition
    configuration::boot_entries();
    // Function to load the kernel from the ESP partition
    kernel::load_kernel();
    println!("---- Ignix bootmanager ----");
    
    // If the input had an error reading the keyboard event, then it will print which error.
    if let Err(e) = input::read_keyboard_events() { // Invokes the function
        println!("Keyboard error: {:?}", e);
    }
    return Status::SUCCESS;
}
