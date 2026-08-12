use std::env;
use std::process::exit;
use sysinfo::{Disks, System};

const B_TO_MB_DIVIDER: u64 = 1048576;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args.get(1).unwrap() == "help" {
        print_help();
        exit(1);
    }

    let mut sys = System::new_all();
    let disks = Disks::new_with_refreshed_list();
    let used_disk = disks.get(0).unwrap();
    sys.refresh_all();

    println!("--System informations--");
    println!("OS: Typewriter OS");
    println!("Kernel: Linux {}", System::kernel_version().unwrap());
    println!(
        "Memory: {} MB /{} MB",
        sys.used_memory() / B_TO_MB_DIVIDER,
        sys.total_memory() / B_TO_MB_DIVIDER
    );
    println!(
        "Disk: {} MB /{} MB",
        used_disk.usage().total_written_bytes / B_TO_MB_DIVIDER,
        used_disk.total_space() / B_TO_MB_DIVIDER,
    )
}
fn print_help() {
    println!("Usage: fetch");
    println!("Show system informations.");
}
