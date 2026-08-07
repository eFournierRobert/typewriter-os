#!/bin/bash

# Pulling TuiText
git clone https://github.com/AchroDev/TuiText.git src/TuiText

# Pulling Linux 7.1.7
mkdir src/linux
curl https://cdn.kernel.org/pub/linux/kernel/v7.x/linux-7.1.7.tar.xz --output src/linux/linux-7.1.7.tar.xz

# Unpacking kernel and putting .config in
cd src/linux
unxz --keep linux-7.1.7.tar.xz && tar -xf linux-7.1.7.tar
cp ../config/kernel.config linux-7.1.7/.config
cd ../..

# Building directory tree
mkdir -p distro/root
cd distro/root
mkdir bin boot dev etc home proc sys tmp
mkdir home/writer
touch home/writer/.history
cd ../..