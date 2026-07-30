# Ignix
## Alpha stage
**BIG DISCLAIMER** - **Ignix** is a **ALPHA STAGE boot manager** written in Rust, **IT IS NOT MEANT TO BE USED IN PRODUCTION AT THIS MOMENT**.
## Table of contents
- [Description](#description)
- [Features](#features)
- [Roadmap](#roadmap)
- [Requirements](#requirements)
- [Installation](#installation)
- [Execution](#execution)
- [Contribution](#contribution)
- [License](#license)
- [Credits](#credits)
## Description
**Ignix** is a **boot manager** written in Rust that aims for **speed** and **stability** during the **boot** process.
## Features
- Minimalist
- Fast
- Stable
- Maintainable and modular
## Roadmap
### TODO (ignix-cli):
- [X] ESP automatic detection

- [X] Installation process and removal

- [X] CRC32 checksum to detect corrupt disks and partitions in the GPT header

- [X] Establishing an ESP directory map

- [ ] Reading the NVRAM variables

- [ ] Modifying the NVRAM variables successfully

- [X] Generating operating system entries

- [ ] Supporting custom signatures of the binary

### TODO (ignix-sdk):
- [X] Secure wrapper and Boot Services working

- [X] Secure wrapper and Runtime Services working

- [X] Secure wrapper and Console Input Protocol working

### TODO (ignix-core):
- [ ] Detect user input

- [ ] Booting any kernel with the initramfs

- [X] Boots the Linux Kernel

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
- A virtual machine snapshot before executing the software
## Installation
Make sure you have the [rustup](https://rust-lang.org/tools/install/) toolchain before trying to build the binary.

Clone the repository and compile: 
```bash
git clone https://github.com/Flamitsu/ignix
cd ignix
cargo core # This command builds the binary for the .efi bin.
cargo cli # This command builds the general binary.
```
However, `cargo core` may produce an error. If that happens, it may be that you don't have the toolchain installed. To install the proper target you need to execute this code:
```bash
cd scripts/
./install-targets.sh 
```
## Execution
### Disclaimer
> This code is still work in progress and it is not meant to be executed in the host machine in any way. You should have a QEMU snapshot (or the software you are using to virtualize an environment) and then execute the software.

If you only want to try the UEFI binary, you need to execute these commands:
```bash
cd scripts/
./only-loader.sh
```

To execute the ignix-cli or ignix-loader binary as a whole, it is **extremely recommended** to be inside a **virtual machine**.

After the installation process is complete, you need to run the following command: `./try-virtual-machine.sh`, and it should be only executed inside a virtual machine. 

## Contribution
To contribute to this project you should look at the [contributing guidelines](https://github.com/Flamitsu/ignix/blob/main/CONTRIBUTING.md) first.
## License
This project is licensed under the [GPL-3.0](https://github.com/Flamitsu/ignix/blob/main/LICENSE)

## Credits
Flamitsu - student 
