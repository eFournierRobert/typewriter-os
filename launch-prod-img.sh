#!/bin/bash
set -e

# Change the firmware is needed. Needs to be UEFI.
BIOS="/usr/share/OVMF/x64/OVMF.4m.fd"

cd build
qemu-system-x86_64 \
    -drive format=raw,file=typewriter.img \
    -bios $BIOS \
    -m 512M