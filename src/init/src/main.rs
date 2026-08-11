//! PID 1 for Typewriter OS.

mod mounts;
mod tty;

use nix::sys::reboot::{reboot, RebootMode};
use nix::unistd::sync;
use std::process::Command;
use std::{env, io};
use std::thread::sleep;
use std::time::Duration;

const SHELL_PATH: &str = "/bin/shell";

fn main() {
    println!("Starting Typewriter OS\nPlease wait...");

    if let Err(e) = mounts::mount_all_filesystems() {
        print_error_message("failed to mount filesystems", &e);
    }

    if let Err(e) = set_env_vars() {
        print_error_message("failed to set up environment variables", &e);
    }

    if let Err(e) = tty::setup_tty() {
        print_error_message("failed to init /dev/tty1", &e);
    }

    println!("Welcome to typewriter OS!");

    let mut shell_process = Command::new(SHELL_PATH)
        .spawn()
        .expect("failed to start shell");
    let shell_res = shell_process.wait();
    
    sync();

    match shell_res {
        Ok(status) => {
            if status.success() {
                let _ = reboot(RebootMode::RB_POWER_OFF);
            } else if status.code().unwrap() == 1 {
                let _ = reboot(RebootMode::RB_AUTOBOOT);
            }
        }
        Err(e) => {
            print_error_message("shell exited unexpectedly", &e);
        }
    }
}

/// Set `$PATH` and change working directory to `/home/writer`.
fn set_env_vars() -> io::Result<()> {
    unsafe {
        env::set_var("PATH", "/bin");
    }
    env::set_current_dir("/home/writer")
}

/// Print an error to stderr and power off the system.
fn print_error_message(m: &str, e: &io::Error) {
    eprintln!("{} : {}", m, e);
    sleep(Duration::from_secs(3));
    let _ = reboot(RebootMode::RB_POWER_OFF);
}
