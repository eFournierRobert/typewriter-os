mod mounts;
mod tty;

use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use std::{env, io};
use std::io::Error;
use nix::sys::reboot;
use nix::sys::reboot::reboot;

fn main() {
    mounts::mount_all_filesystems();
    unsafe {
        println!("setting up environment variables");
        if let Err(e) = set_env_vars() {
            print_error_message("failed to set up environment variables", &e);
        }

        println!("opening tty1");
        if let Err(e) = tty::setup_tty() {
            print_error_message("failed to init /dev/tty1", &e);
        }
    }

    println!("welcome to typewriter OS!");

    let mut shell_process = Command::new("/bin/shell")
        .spawn()
        .expect("failed to start shell");
    let _ = shell_process.wait();

    let _ = reboot(reboot::RebootMode::RB_POWER_OFF);
}

unsafe fn set_env_vars() -> io::Result<()> {
    unsafe {
        env::set_var("PATH", "/bin");
    }
    env::set_current_dir("/home/writer")
}

fn print_error_message(m: &str, e: &Error) {
    eprintln!("{} : {}", m, e);
    loop {
        sleep(Duration::from_secs(5));
    }
}
