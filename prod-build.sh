#!/bin/bash

set -e

FILE_NAME="typewriter.img"

# Build necessary files
echo "Building necessary files..."
./build-and-run.sh none

# Create build directory
echo "Creating build directory..."
mkdir build
cd build
mkdir bootmnt
mkdir rootmnt

# Creating and formatting image file
echo "Creating and formatting image file..."
truncate -s 1G $FILE_NAME

sudo sfdisk $FILE_NAME <<EOF
label: gpt
size=512M, type=U
type=L
EOF

sudo losetup --find --partscan typewriter.img
sudo mkfs.vfat /dev/loop0p1
sudo mkfs.ext4 /dev/loop0p2

# Mounting and copying files over
echo "Mounting and copying to image..."
sudo mount /dev/loop0p1 bootmnt
sudo mount /dev/loop0p2 rootmnt

sudo cp ../distro/initramfz bootmnt/
sudo cp ../distro/vmlinuz-linux bootmnt/

sudo touch rootmnt/.history

# Creating grub
echo "Creating everything grub related..."
sudo mkdir bootmnt/grub
sudo cp ../src/config/grub.cfg bootmnt/grub/

sudo grub-install \
--target=x86_64-efi \
--efi-directory=bootmnt/ \
--boot-directory=bootmnt/ \
--removable

# Synching and cleaning up
echo "Synching and cleaning up"
sync
sudo umount bootmnt rootmnt
sudo losetup -d loop0

echo "Done creating prod image!"