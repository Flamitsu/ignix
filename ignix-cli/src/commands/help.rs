pub fn show_help() {
    const HELP_TEXT: &str = r#"
Ignix: A minimalist EFI boot manager written in Rust.

USAGE:
    ignix [COMMAND] [OPTIONS]

COMMANDS:
    install     Installs ignix binary into the EFI System Partition
    remove      Removes ignix and its configuration from the ESP
    list        List all the current entries in the system
    check       Check the current configured entries and their integrity
    update      Synchronizes kernel entries and updates boot configuration
    clean       Removes invalid boot entries
    help        Prints this help information

OPTIONS:
    -y, --yes               Skip all interactive confirmation prompts
    --efi-bin=[PATH]        Manual path to the EFI binary (default: auto-detect)
    --no-nvram              Skips all the logic to write a NVRAM variable.
    --allow-virtual         Allows to install the .efi bin in a virtual device.
    --install-route=[PATH]  Skips the auto-detect of the ESP partition and installs the .efi there
    --removable             Allows to install the .efi bin in a removable device. 
"#;
    println!("{}", HELP_TEXT.trim());
}
