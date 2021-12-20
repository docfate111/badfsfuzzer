pub const BTRFS_SUPERBLOCK_MAGIC: [u8; 8] = *b"_BHRfS_M";

mod btrfs_parse {
    use memmap::{Mmap, MmapOptions};
    use std::fs::File;
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

    /*fn parse_block(memmapd: &Mmap) -> Vec<&[u8]> {
        let magic_offsets: [u32; 3] =         
    }

    pub fn extract(input_name: &'a str, output_name: &'a str) -> Result<(), Error> {
    
        Ok(())
    }

    pub fn insert(metadata: &str, file_to_insert: &str) -> {

    }*/
}
