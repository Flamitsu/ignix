#!/usr/bin/bash
# This script will boot up a virtual machine with the structure needed for the .EFI to work. (only .EFI)
BOOT_DIR="../boot-test"
DEPENDENCIES_DIR="/usr/share/edk2-ovmf"
# If the directory does not exists then it will try to setup it.
if [ ! -d "$BOOT_DIR" ]; then
    mkdir "$BOOT_DIR"
    cp "$DEPENDENCIES_DIR/OVMF_CODE.fd" "$BOOT_DIR/" || { echo "Are edk2-ovmf tools installed ? OVMF_CODE.fd not found in '$DEPENDENCIES_DIR'"; exit 1; }
    cp "$DEPENDENCIES_DIR/OVMF_VARS.fd" "$BOOT_DIR/" || { echo "Are edk2-ovmf tools installed ? OVMF_VARS.fd not found in '$DEPENDENCIES_DIR'"; exit 1; }
    mkdir -p "$BOOT_DIR/esp/efi/boot"
fi 
# Will try to build the binary
cargo loader || { echo "You may need to run the 'install-targets.sh' script in this directory."; exit 1; }
# Will try to copy the recently compiled binary
cp ../target/x86_64-unknown-uefi/debug/ignixx64.efi "$BOOT_DIR/esp/efi/boot/bootx64.efi"

DEBUG_FLAGS=""
if [ "$1" == "--debug" ]; then
    echo "QEMU waiting LLDB in port 1234..."
    DEBUG_FLAGS="-s -S"
fi

# Will try to run the script. 
(
    cd ../boot-test || { echo "Could not change to the $BOOT_DIR directory."; exit 1; }
    qemu-system-x86_64 -enable-kvm \
        -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
        -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
        -drive format=raw,file=fat:rw:esp \
        $DEBUG_FLAGS || { echo ""; exit 1; }
)


