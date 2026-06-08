// SPDX-License-Identifier: GPL-3.0-only
pub fn show_help() {
    const HELP_TEXT: &str = r#"
USAGE:
    ignix [COMMAND] [OPTIONS]

COMMANDS:
    add         Adds a new entry into the ESP directory.
    help        Prints this help information
    hook        Helps the bash install hook with some commands.
    install     Installs ignix binary into the EFI System Partition
    uninstall   Removes ignix binary and its configuration from the ESP

OPTIONS:
    --force                 Skip all interactive confirmation prompts. Only use if you know what you're doing.
    --efi-bin=[PATH]        Manual path to the EFI binary (default: auto-detect)
    --no-nvram              Skips all the logic to write a NVRAM variable.
    --allow-virtual         Allows to install the .efi bin in a virtual device.
    --removable             Allows to install the .efi bin in a removable device.
    --get-esp-mountpoint    Gets the ESP mountpoint for the hook command
    --get-machine-id        Gets the machine-id for the hook command.
"#;
    println!("{}", HELP_TEXT.trim());
}
