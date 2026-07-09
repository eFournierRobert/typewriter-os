#!/bin/bash

# Build and copy init
cd src/init
cargo build --release --target=x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/init ../../distro/root/
cd ../..

# Build and copy shell
cd src/shell
cargo build -r --target=x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/shell ../../distro/root/bin

cd ../..

# Build and copy create
cd src/core-utils/create
cargo build -r --target=x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/create ../../../distro/root/bin

cd ../../..

# Build and copy list
cd src/core-utils/list
cargo build -r --target=x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/list ../../../distro/root/bin

cd ../../..

# Build and copy copy
cd src/core-utils/copy
cargo build -r --target=x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/copy ../../../distro/root/bin

cd ../../..

# Build and copy delete
cd src/core-utils/delete
cargo build -r --target=x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/delete ../../../distro/root/bin

cd ../../..

# Build and copy find
cd src/core-utils/find
cargo build -r --target=x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/find ../../../distro/root/bin

cd ../../..

# Build and copy mve
cd src/core-utils/mve
cargo build -r --target=x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/move ../../../distro/root/bin

cd ../../..

# Build and copy new
cd src/core-utils/new
cargo build -r --target=x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/new ../../../distro/root/bin

cd ../../..

# Build and copy rename
cd src/core-utils/rename
cargo build -r --target=x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/rename ../../../distro/root/bin

cd ../../..

# Build and copy search
cd src/core-utils/search
cargo build -r --target=x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/search ../../../distro/root/bin

cd ../../..

# Build and copy show
cd src/core-utils/show
cargo build -r --target=x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/show ../../../distro/root/bin

cd ../../..

# Create initramfs filesystem
cd distro/root
find . | cpio -ov --format=newc | gzip -9 >../initramfz
cd ../..

# Run the initramfs
if [ $1 = "term" ]; then
    qemu-system-x86_64 -kernel /boot/vmlinuz-linux --nographic -append "console=ttyS0" -initrd distro/initramfz
else
    qemu-system-x86_64 -kernel /boot/vmlinuz-linux -initrd distro/initramfz
fi
