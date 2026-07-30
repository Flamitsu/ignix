#!/usr/bin/bash
BOOT_DIR="../boot-test"
IMG_FILE="$BOOT_DIR/disk.img"
ESP_DIR="$BOOT_DIR/esp"
mkdir -p "$ESP_DIR/efi/boot"
if [ ! -f "$BOOT_DIR/OVMF_CODE.fd" ]; then
    POSSIBLE_DIRS=(
        "/usr/share/edk2-ovmf"
        "/usr/share/edk2/x64"
        "/usr/share/ovmf/x64"
        "/usr/share/OVMF"
    )
    FOUND_CODE=""
    FOUND_VARS=""
    for dir in "${POSSIBLE_DIRS[@]}"; do
        for suffix in "" ".4m" ".2m"; do
            if [ -f "$dir/OVMF_CODE${suffix}.fd" ] && [ -f "$dir/OVMF_VARS${suffix}.fd" ]; then
                FOUND_CODE="$dir/OVMF_CODE${suffix}.fd"
                FOUND_VARS="$dir/OVMF_VARS${suffix}.fd"
                break 2
            fi
        done
    done
    if [ -n "$FOUND_CODE" ]; then
        cp "$FOUND_CODE" "$BOOT_DIR/OVMF_CODE.fd"
        cp "$FOUND_VARS" "$BOOT_DIR/OVMF_VARS.fd"
    else
        echo "Error: The script didn't find OVMF_CODE or OVMF_VARS in your system."
        echo "Please, ensure that you have installed the edk2-ovmf package of your distribution."
        exit 1
    fi
fi
cargo core || { echo "Error while compiling"; exit 1; }
cp ../target/x86_64-unknown-uefi/debug/ignixx64.efi "$ESP_DIR/efi/boot/bootx64.efi"
dd if=/dev/zero of="$IMG_FILE" bs=1M count=400 status=none
mformat -i "$IMG_FILE" -F -v IGNIX_ESP ::
mcopy -s -o -i "$IMG_FILE" "$ESP_DIR"/* ::/
DEBUG_FLAGS=""
if [ "$1" == "--debug" ]; then
    echo "QEMU waiting LLDB on port 1234..."
    DEBUG_FLAGS="-s -S"
fi
(
    cd "$BOOT_DIR" || { echo "Couldn't change directories"; exit 1; }
    qemu-system-x86_64 -enable-kvm -m 2G \
        -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
        -drive if=pflash,format=raw,readonly=off,file=OVMF_VARS.fd \
        -drive format=raw,file=disk.img \
        $DEBUG_FLAGS || { echo ""; exit 1; }
)
