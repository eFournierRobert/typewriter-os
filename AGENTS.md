# typewriter-os — Agent Instructions

## What this is

Typewriter OS — a Linux-based operating system with almost the entire userspace written from scratch in Rust. Booted as an initramfs inside QEMU using the host kernel (`/boot/vmlinuz-linux`). No package manager — just binaries packed into a cpio archive.

## Agent role

You assist with exactly two tasks:

1. **Documentation** — Document functions and methods across the codebase.
2. **Code & refactor suggestions** — Provide recommendations to improve code quality, structure, and idiomatic Rust.

You must not write or modify code unless explicitly instructed to do so. The user handles all code writing; your role is to assist, suggest, and document.

## Build + run

```bash
bash build-and-run.sh          # GUI (requires display)
bash build-and-run.sh term     # serial console (no display needed)
```

The script must be run from the repo root. It:
1. Cross-compiles every crate to `x86_64-unknown-linux-musl` (release only).
2. Copies binaries into `distro/root/`.
3. Packages `distro/root/` into `distro/initramfz` via `cpio | gzip`.
4. Boots it with QEMU.

Single Cargo workspace at root (`Cargo.toml`). Run `cargo build` from the repo root — all crates share one `target/` directory.

## Crate layout

| Directory | Role |
|---|---|
| `src/init/` | PID 1 — mounts filesystems, sets env, spawns `/bin/shell`, then powers off (exit code 0) or reboots (exit code 1) based on shell exit status |
| `src/shell/` | User-facing CLI shell (input loop, history, command dispatch) |
| `src/core-utils/*/` | Individual binaries: `create`, `list`, `copy`, `delete`, `find`, `new`, `show`, `search`, `rename`, `mve`, `fetch` |
| `src/core-utils/jail_check/` | Shared library — validates that paths resolve within `/home/writer` before operations are performed |

The `mve` crate produces a binary named `move` (reserved keyword workaround via `[[bin]] name = "move"`).

## Key constraints

- **All binaries must compile to `x86_64-unknown-linux-musl`.** Install the target with `rustup target add x86_64-unknown-linux-musl` if missing.
- **Edition 2024** across all crates.
- `distro/` is gitignored — never commit built artifacts.
- QEMU uses the host's kernel; it must exist at `/boot/vmlinuz-linux`.
- The init process sets `PATH=/bin` and cwd to `/home/writer`. All binaries live in `/bin`.
- **Jail enforcement:** Core utilities must validate paths via `jail_check::is_target_valid` before operating. This ensures all file operations stay within `/home/writer`, preventing access outside the permitted directory tree.
- **Reboot support:** The shell exits with code 0 for poweroff (`exit` / `poweroff`) and code 1 for reboot (`reboot`). Init reads this status to decide whether to call `RB_POWER_OFF` or `RB_AUTOBOOT`.
- **Persistent storage:** Both QEMU modes (GUI and term) attach `distro/datadisk.img` as a raw drive, mounted at `/home/writer` by init.
