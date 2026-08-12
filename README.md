# Typewriter OS

Typewriter OS is an embedded Linux distribution with userspace built from scratch in Rust. It was made
to run in x86_64 UEFI QEMU.

The userspace is a CLI that allows for persistent CRUD actions on files and directories. The user is jailed
inside the persistent directory.

## Core userspace programs
Here's a list of core userspace programs part of the OS :
- Init system
- Minimal shell
- [TuiText by AnchroDev](https://github.com/AchroDev/TuiText)

## Core-utils programs
Here's a list of core-utils made for the OS :
- copy
- create
- delete
- fetch
- find
- list
- move
- new
- rename
- search
- show

## Quick start
The repository has some scripts to initialize, build and launch the image for you. The prerequisite are :
- A Linux system
- xz
- git
- cargo/rust
- make
- gzip
- cpio
- x86_64 QEMU with a UEFI firmware
- grub

Here's a one line to initialize, build and launch the production image.
> [!WARNING]
> This action will compile an x86_64 Linux kernel from scratch so this might 
> take a bit of time and use a lot of CPU.
```bash
./init.sh && ./prod-build.sh && ./launch-prod-img.sh
```
