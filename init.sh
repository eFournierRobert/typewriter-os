#!/bin/bash

set -e

# Pulling TuiText
echo "Cloning TuiText from Github"
git clone https://github.com/AchroDev/TuiText.git src/TuiText

# Pulling Linux 7.1.7
echo "Fetching Linux 7.1.7..."
mkdir src/linux
curl https://cdn.kernel.org/pub/linux/kernel/v7.x/linux-7.1.7.tar.xz --output src/linux/linux-7.1.7.tar.xz

# Unpacking kernel and putting .config in
echo "Decompresssing kernel..."
cd src/linux
unxz --keep linux-7.1.7.tar.xz && tar -xf linux-7.1.7.tar
cp ../config/kernel.config linux-7.1.7/.config
cd ../..

# Building directory tree
echo "Building root directory tree..."
mkdir -p distro/root
cd distro/root
mkdir -p bin boot dev etc home/writer proc sys tmp
cd ..

echo "Building dev datadisk.img..."
truncate -s 1G datadisk.img
sudo sfdisk datadisk.img <<EOF
label: gpt
type=L
EOF

sudo losetup --find --partscan datadisk.img
sudo mkfs.ext4 /dev/loop0p1
sudo losetup -d loop0
cd ..

echo "Done!"
