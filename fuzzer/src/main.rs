use std::env::{args, Args};
use std::fs::File;
use memmap::MmapOptions;
//use std::io::Write;
const BTRFS_SUPERBLOCK_MAGIC: [u8; 8] = *b"_BHRfS_M";

fn main() -> Result<(), &'static str> {
	let mut args: Args = args();
	let filename = match args.nth(1) {
    		None => {
			    println!("Usage: ./fuzzer [filesystem image]");
			    return Err("invalid usage");
			},
    		Some(value) => value,
	};
	let file = match File::open(filename) {
            Err(_) => {
                return Err("opening file");
            },
            Ok(f) => f,
    };
    let mmap = unsafe { 
        match MmapOptions::new().map(&file) {
            Err(_) => {
                return Err("mmap error");
            },
            Ok(f) => f,
        }
    };
    let mut magic = String::new();
    for i in BTRFS_SUPERBLOCK_OFFSET..BTRFS_SUPERBLOCK_OFFSET+100 { 
            let x = match mmap.get(i) {
                Some(n) => *n,
                None => 0
            };
            magic.push(x as char);
    }
    println!("{}", magic);
    Ok(())
}
