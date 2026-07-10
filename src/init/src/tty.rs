use libc::{
    O_RDWR, STDERR_FILENO, STDIN_FILENO, STDOUT_FILENO, close, dup2, fchmod, fchown, ioctl, open,
    setsid,
};
use std::ffi::CString;
use std::io::Error;

pub unsafe fn setup_tty() {
    let tty = CString::new("/dev/tty1").unwrap();

    unsafe {
        assert!(setsid() >= 0, "setsid failed: {}", Error::last_os_error());
        let tty_fd = open(tty.as_ptr(), O_RDWR);
        if tty_fd >= 0 {
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
