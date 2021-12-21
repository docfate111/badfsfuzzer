pub mod btrfs_parse {
    use memmap::{Mmap, MmapOptions};
    use std::fs::File;
    use std::io::Write;
    pub const BTRFS_SUPERBLOCK_MAGIC: [u8; 8] = *b"_BHRfS_M";
    fn map_to_file(filename: &str) -> Result<Mmap, &'static str> {  
        let file = match File::open(filename) {
            Err(_) => { 
                return Err("opening file");
            },
            Ok(f) => f,
        };
        return unsafe {  
            match MmapOptions::new().map(&file) {
                Err(_) => {
                    return Err("mmap error");
                },
                Ok(f) => Ok(f),
            }
        };
    }

    fn parse_block(memmapd: &Mmap, out: &mut File) -> bool {
        let magic_offsets: [usize; 3] = [0x10_040, 0x4_000_040, 0x4_000_000_040];
        let mut parsed = false;
        for offset in magic_offsets {
            match memmapd.get(offset..offset+8) {
                Some(v) => {
                    if v == BTRFS_SUPERBLOCK_MAGIC {
                        println!("Found magic btrfs header at {:#01x}", offset);
                        let start = offset - 0x40;
                        match memmapd.get(start .. start + 0xdcb) {
                            Some(block) => {
                                parsed = true;
                                println!("Writing superblock");
                                out.write(block);
                            },
                            None => {
                                println!("Not writing metadata");
                                continue;
                            },
                        }
                    }
                },
                None => {
                    continue;
                }
            }
        }
        parsed
    }

    pub fn extract<'a>(input_name: &'a str, out_name: &'a str) -> Result<(), &'static str> {
        let memmapd = match map_to_file(input_name) {
            Err(s) => { return Err(s); },
            Ok(f) => f,
        };
        let mut path = "./corpus/".to_owned();
        path.push_str(out_name);
        let mut file = File::create(path).expect("Error creating file");
        if parse_block(&memmapd, &mut file) {
            println!("Extracted to {:}", out_name);
            return Ok(());
        }
        Err("Error extracting metadata")
    }

    /*pub fn insert(metadata: &str, file_to_insert: &str) -> {

    }*/
}
