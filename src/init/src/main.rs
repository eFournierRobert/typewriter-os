use libc::{
    LINUX_REBOOT_CMD_POWER_OFF, O_RDWR, STDERR_FILENO, STDIN_FILENO, STDOUT_FILENO, close, dup2,
    fchmod, fchown, ioctl, open, reboot, setsid,
};
use std::ffi::CString;
use std::io::Error;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use std::{env, io};
use sys_mount::{FilesystemType, Mount};

fn main() {
    mount_all_filesystems();
    unsafe {
        println!("setting up environment variables");
        if let Err(e) = set_env_vars() {
            print_error_message("failed to set up environment variables", e);
        }

        println!("opening tty1");
        setup_tty();
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

fn mount_all_filesystems() {
    let dev_result = Mount::builder()
        .fstype(FilesystemType::from("devtmpfs"))
        .mount("devtmpfs", "/dev");
    let proc_result = Mount::builder()
        .fstype(FilesystemType::from("proc"))
        .mount("proc", "/proc");
    let sysfs_result = Mount::builder()
        .fstype(FilesystemType::from("sysfs"))
        .mount("sysfs", "/sys");
    let tmpfs_result = Mount::builder()
        .fstype(FilesystemType::from("tmpfs"))
        .mount("tmpfs", "/tmp");

    was_mount_successful_check(dev_result);
    was_mount_successful_check(proc_result);
    was_mount_successful_check(sysfs_result);
    was_mount_successful_check(tmpfs_result);
}

fn was_mount_successful_check(res: Result<Mount, Error>) {
    match res {
        Ok(mount) => {
            println!(
                "Mounted {} successfully {}",
                mount.get_fstype(),
                mount.target_path().to_string_lossy()
            );
        }
        Err(err) => {
            print_error_message("failed to mount", err);
        }
    }
}

unsafe fn setup_tty() {
    let tty = CString::new("/dev/tty1").unwrap();

    unsafe {
        assert!(setsid() >= 0, "setsid failed: {}", Error::last_os_error());
        let tty_fd = open(tty.as_ptr(), O_RDWR);
        if (tty_fd >= 0) {
            assert!(
                ioctl(tty_fd, libc::TIOCSCTTY, 0) >= 0,
                "ioctl failed: {}",
                Error::last_os_error()
            );
            assert!(
                fchown(tty_fd, 0, 0) >= 0,
                "fchown failed: {}",
                Error::last_os_error()
            );
            assert!(
                fchmod(tty_fd, 0666) >= 0,
                "fchmod failed: {}",
                Error::last_os_error()
            );

            assert!(
                dup2(tty_fd, STDIN_FILENO) >= 0,
                "dup2 failed: {}",
                Error::last_os_error()
            );
            assert!(
                dup2(tty_fd, STDOUT_FILENO) >= 0,
                "dup2 failed: {}",
                Error::last_os_error()
            );
            assert!(
                dup2(tty_fd, STDERR_FILENO) >= 0,
                "dup2 failed: {}",
                Error::last_os_error()
            );

            assert!(
                close(tty_fd) >= 0,
                "close failed: {}",
                Error::last_os_error()
            );
        }
    }
}

fn print_error_message(m: &str, e: Error) {
    eprintln!("{} : {}", m, e);
    loop {
        sleep(Duration::from_secs(5));
    }
}
