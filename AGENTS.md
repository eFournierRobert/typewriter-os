# typewriter-os — Agent Instructions

## What this is

A hand-rolled Linux OS written in Rust. Booted as an initramfs inside QEMU using the host kernel (`/boot/vmlinuz-linux`). No package manager, no system libraries — just musl-static binaries packed into a cpio archive.

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
| `src/init/` | PID 1 — mounts filesystems, sets env, spawns `/bin/shell`, then powers off |
| `src/shell/` | User-facing CLI shell (input loop, history, command dispatch) |
| `src/core-utils/*/` | Individual binaries: `create`, `list`, `copy`, `delete`, `find`, `new`, `show`, `search`, `rename`, `mve` |

The `mve` crate produces a binary named `move` (reserved keyword workaround via `[[bin]] name = "move"`).

## Key constraints

- **All binaries must compile to `x86_64-unknown-linux-musl`.** Install the target with `rustup target add x86_64-unknown-linux-musl` if missing.
- **Edition 2024** across all crates.
- `distro/` is gitignored — never commit built artifacts.
- QEMU uses the host's kernel; it must exist at `/boot/vmlinuz-linux`.
- The init process sets `PATH=/bin` and cwd to `/home/writer`. All binaries live in `/bin`.
