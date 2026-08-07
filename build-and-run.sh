#!/bin/bash

# Build whole workspace
cargo build --release --target=x86_64-unknown-linux-musl

# Copy binaries
cp target/x86_64-unknown-linux-musl/release/init distro/root/
cp target/x86_64-unknown-linux-musl/release/shell distro/root/bin
cp target/x86_64-unknown-linux-musl/release/create distro/root/bin
cp target/x86_64-unknown-linux-musl/release/list distro/root/bin
cp target/x86_64-unknown-linux-musl/release/copy distro/root/bin
cp target/x86_64-unknown-linux-musl/release/delete distro/root/bin
cp target/x86_64-unknown-linux-musl/release/find distro/root/bin
cp target/x86_64-unknown-linux-musl/release/move distro/root/bin
cp target/x86_64-unknown-linux-musl/release/new distro/root/bin
cp target/x86_64-unknown-linux-musl/release/rename distro/root/bin
cp target/x86_64-unknown-linux-musl/release/search distro/root/bin
cp target/x86_64-unknown-linux-musl/release/show distro/root/bin
cp target/x86_64-unknown-linux-musl/release/TuiText distro/root/bin/tuitext

# Build and copy Linux kernel
cd src/linux/linux-7.1.7
make -j$(nproc)
cp arch/x86/boot/bzImage ../../../distro/root/boot/vmlinuz-linux
cd ../../..

# Create initramfs filesystem
cd distro/root
find . | cpio -ov --format=newc | gzip -9 >../initramfz
cd ../..

# Run the initramfs
if [ $1 = "term" ]; then
    qemu-system-x86_64 -kernel distro/root/boot/vmlinuz-linux --nographic -append "console=ttyS0" -initrd distro/initramfz
else
    qemu-system-x86_64 -kernel distro/root/boot/vmlinuz-linux -initrd distro/initramfz
fi
