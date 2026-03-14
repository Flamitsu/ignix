/// This function is supposed to show the help aviable for the execution of spark
pub fn show_help() {
    const HELP_TEXT: &str = r#"
Spark: A minimalist EFI boot manager written in Rust.

USAGE:
    spark [COMMAND] [OPTIONS]

COMMANDS:
    install     Installs spark binary into the EFI System Partition
    remove      Removes spark and its configuration from the ESP
    list        List all the current entries in the system
    check       Check the current configured entries and their integrity
    update      Synchronizes kernel entries and updates boot configuration
    clean       Removes invalid boot entries
    help        Prints this help information

OPTIONS:
    -y, --yes               Skip all interactive confirmation prompts
    --efi-bin=[PATH]        Manual path to the EFI binary (default: auto-detect)

EXAMPLES:
    spark install --yes
    spark install --efi-bin=/usr/lib/spark/spark.efi
    spark update
"#;
    println!("{}", HELP_TEXT.trim());
}
