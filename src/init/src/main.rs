mod mounts;
mod tty;

use nix::sys::reboot::{self, reboot};
use std::process::Command;
use std::{env, io};

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

    let mut shell_process = Command::new("/bin/shell")
        .spawn()
        .expect("failed to start shell");
    let _ = shell_process.wait();

    let _ = reboot(reboot::RebootMode::RB_POWER_OFF);
}

fn set_env_vars() -> io::Result<()> {
    unsafe {
        env::set_var("PATH", "/bin");
    }
    env::set_current_dir("/home/writer")
}

fn print_error_message(m: &str, e: &io::Error) {
    eprintln!("{} : {}", m, e);
    let _ = reboot(reboot::RebootMode::RB_POWER_OFF);
}
