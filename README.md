# Spark
## Pre alpha stage
**BIG DISCLAIMER** - **Spark** is a **PRE ALPHA STAGE boot manager** written in Rust, **IT IS NOT MEANT TO BE USED IN PRODUCTION AT THIS MOMENT**.
## Table of contents
- [Description](#description)
- [Features](#features)
- [TODO](#todo)
- [Requirements](#requirements)
- [Installation](#installation)
- [Execution](#execution)
- [Contribution](#contribution)
- [License](#license)
- [Credits](#credits)
## Description
**Spark** is a **boot manager** written in Rust that aims for **speed** and **stability** during the **boot** process.
## Features
- Minimalist
- Fast
- Stable
- Maintainable and modular
## TODO
### TODO (general binary):
- [X] ESP automatic detection

- [X] Installation process and removal

- [X] Establishing an ESP directory map

- [ ] Reading the NVRAM variables

- [ ] Modifying the NVRAM variables successfully

- [ ] Generating operating system entries

- [ ] Supporting custom signatures of the binary
### TODO (UEFI binary):
- [X] Detect user input

- [ ] Booting any kernel an initramfs

- [ ] Supporting firmware signatures

- [ ] Choose between entries

- [ ] Parsing the system's entries in the ESP

- [ ] Customization of the general config file
## Requirements
- UEFI firmware
- GPT partition table
- Rustup configured
- x86_64 architecture
- QEMU installed and configured with a Linux OS instance
- QEMU virtual snapshot of the virtual machine before executing the software
## Installation
Make sure you have the [rustup](https://rust-lang.org/tools/install/) toolchain before trying to build the binary.

Clone the repository and compile: 
```bash
git clone https://github.com/Flamitsu/spark
cd spark
cargo loader # This command builds the binary for the .efi bin.
cargo cli # This command builds the general binary.
```
However, `cargo loader` may produce an error. If that happens, it may be that you don't have the toolchain installed. 
To proceed execute: `rustup target add x86_64-unknown-uefi` and re-run: `cargo loader` command.
## Execution
### Disclaimer
> This code is still work in progress and it is not meant to be executed in the host machine in any way. You should have a QEMU snapshot (or the software you are using to virtualize an environment) and then execute the software.

To execute the binary, it is **extremely recommended** to be inside a **virtual machine**.

After the installation process is complete, you need to run the following command: `./execute.sh`, reboot the virtual machine and it should boot the EFI binary.

## Contribution
To contribute to this project you should look at the [contributing guidelines](https://github.com/Flamitsu/spark/blob/main/CONTRIBUTING.md) first.
## License
This project is licensed under the [MIT](https://github.com/Flamitsu/spark/blob/main/LICENSE-MIT) license or under the [Apache 2.0 license](https://github.com/Flamitsu/spark/blob/main/LICENSE-APACHE)
## Credits
- [uefi](https://github.com/rust-osdev/uefi-rs/tree/main) - Crate to interact with the UEFI in Rust.
