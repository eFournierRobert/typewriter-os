//! Mount essential virtual filesystems.

use std::io::Error;
use sys_mount::{FilesystemType, Mount};

/// Mount `/dev`, `/proc`, `/sys`, and `/tmp`.
pub fn mount_all_filesystems() -> Result<(), Error> {
    let mut drives = Vec::new();
    drives.push(("devtmpfs", "/dev"));
    drives.push(("proc", "/proc"));
    drives.push(("sysfs", "/sys"));
    drives.push(("tmpfs", "/tmp"));

    for (fs, dest) in drives {
        println!("mounting {}...", fs);
        let _ = Mount::builder()
            .fstype(FilesystemType::from(fs))
            .mount(fs, dest)?;
    }

    let dev_data_partition = Mount::builder()
        .fstype("ext4")
        .mount("/dev/sda1", "/home/writer");

    if dev_data_partition.is_err() {
        let _ = Mount::builder()
            .fstype("ext4")
            .mount("/dev/sda2", "/home/writer")?;
    }
    Ok(())
}
