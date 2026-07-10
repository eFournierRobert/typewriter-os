mod mounts;
mod tty;

use libc::{LINUX_REBOOT_CMD_POWER_OFF, reboot};
use std::io::Error;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use std::{env, io};

fn main() {
    mounts::mount_all_filesystems();
    unsafe {
        println!("setting up environment variables");
        if let Err(e) = set_env_vars() {
            print_error_message("failed to set up environment variables", e);
        }

        println!("opening tty1");
        tty::setup_tty();
    }

    println!("welcome to typewriter OS!");

    let mut shell_process = Command::new("/bin/shell")
        .spawn()
        .expect("failed to start shell");
    shell_process.wait();

    unsafe {
        reboot(LINUX_REBOOT_CMD_POWER_OFF);
    }
}

unsafe fn set_env_vars() -> io::Result<()> {
    unsafe {
        env::set_var("PATH", "/bin");
    }
    env::set_current_dir("/home/writer")
}

fn print_error_message(m: &str, e: Error) {
    eprintln!("{} : {}", m, e);
    loop {
        sleep(Duration::from_secs(5));
    }
}
