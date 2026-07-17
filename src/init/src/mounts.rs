use std::collections::HashMap;
use std::io::Error;
use sys_mount::{FilesystemType, Mount};

pub fn mount_all_filesystems() -> Result<(), Error> {
    let mut drives = HashMap::new();
    drives.insert("devtmpfs", "/dev");
    drives.insert("proc", "/proc");
    drives.insert("sysfs", "/sys");
    drives.insert("tmpfs", "/tmp");

    for (fs, dest) in drives {
        let _ = Mount::builder()
            .fstype(FilesystemType::from(fs))
            .mount(fs, dest)?;
    }
    Ok(())
}
