use std::io::Error;
use std::os::fd::AsRawFd;
use std::os::unix::fs::fchown;
use libc::{ioctl};
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::unistd::{close, dup2_stderr, dup2_stdin, dup2_stdout, setsid};

const TTY_PATH: &str = "/dev/tty1";
const TTY_RIGHTS_MODE: Mode = Mode::from_bits_retain(0o666);
const TTY_UID_GID: Option<u32> = Some(0);

pub fn setup_tty() -> Result<(), Error>{
    setsid()?;

    let tty_fd = open(TTY_PATH, OFlag::O_RDWR, TTY_RIGHTS_MODE)?;

    unsafe {
        if ioctl(tty_fd.as_raw_fd(), libc::TIOCSCTTY, 0) < 0 {
            return Err(Error::last_os_error());
        }
    };

    fchown(&tty_fd, TTY_UID_GID, TTY_UID_GID)?;

    dup2_stdin(&tty_fd)?;
    dup2_stdout(&tty_fd)?;
    dup2_stderr(&tty_fd)?;

    close(tty_fd)?;

    Ok(())
}
