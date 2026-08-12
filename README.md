# Typewriter OS

Typewriter OS is an embedded Linux distribution with a userspace built from scratch in musl Rust. It runs on x86_64 UEFI QEMU.

The OS was made for simple file editing and CRUD actions on them. This allows it to be
a small, purpose-built CLI typing machine.

## Core userspace programs
Here's a list of core userspace programs part of the OS:
- Init system
- Minimal shell
- [TuiText by AnchroDev](https://github.com/AchroDev/TuiText)

## Core-utils programs
Here's a list of core-utils made for the OS:
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
The repository has some scripts to initialize, build and launch the image for you. The prerequisites are:
- A Linux system
- xz
- git
- cargo
- make
- gzip
- cpio
- x86_64 QEMU with a UEFI firmware
- grub
- sudo
- rustup target `x86_64-unknown-linux-musl`

Here's a one liner to initialize, build and launch the production image.
> [!WARNING]
> **This action will compile an x86_64 Linux kernel so this might 
> take a bit of time and use a lot of CPU.**
```bash
./init.sh && ./prod-build.sh && ./launch-prod-img.sh
```
## Development script
During development, use the faster script `build-and-run.sh`. It doesn't create a full bootable production image.

Script arguments:
- `none`

  Only builds the packages and doesn't run.
- `term`

  Attaches `ttyS0` to the current terminal. Helps with seeing kernel panics.
- No args

  Runs a QEMU VM in a new dedicated window.
