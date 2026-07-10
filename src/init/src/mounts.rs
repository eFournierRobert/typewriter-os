use crate::print_error_message;
use std::io::Error;
use sys_mount::{FilesystemType, Mount};

pub fn mount_all_filesystems() {
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

    handle_mount_result(dev_result);
    handle_mount_result(proc_result);
    handle_mount_result(sysfs_result);
    handle_mount_result(tmpfs_result);
}

fn handle_mount_result(res: Result<Mount, Error>) {
    match res {
        Ok(mount) => {
            println!(
                "Mounted {} successfully {}",
                mount.get_fstype(),
                mount.target_path().to_string_lossy()
            );
        }
        Err(err) => {
            print_error_message("failed to mount", &err);
        }
    }
}
