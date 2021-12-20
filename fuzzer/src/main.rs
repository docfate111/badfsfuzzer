use libafl::{
    bolts::{current_nanos, rands::StdRand, tuples::tuple_list},
    corpus::{InMemoryCorpus, OnDiskCorpus},
    feedbacks::{CrashFeedback, MapFeedbackState, MaxMapFeedback},
    fuzzer::StdFuzzer,
    inputs::BytesInput,
    mutators::{havoc_mutations, Mutator, StdScheduledMutator},
    observers::StdMapObserver,
    state::StdState,
};
use memmap::{Mmap, MmapOptions};
use std::env::args;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
//use std::io::prelude::*;
const BTRFS_CSUM_SIZE: usize = 32;
const BTRFS_LABEL_SIZE: usize = 256;
const BTRFS_FSID_SIZE: usize = 16;
const BTRFS_UUID_SIZE: usize = 16;
const BTRFS_SYSTEM_CHUNK_ARRAY_SIZE: usize = 2048;

const BTRFS_SUPERBLOCK_MAGIC: [u8; 8] = *b"_BHRfS_M";
pub const BTRFS_FS_TREE_OBJECTID: u64 = 5;

pub const BTRFS_INODE_REF_KEY: u8 = 12;
pub const BTRFS_DIR_ITEM_KEY: u8 = 84;
pub const BTRFS_ROOT_ITEM_KEY: u8 = 132;
pub const BTRFS_CHUNK_ITEM_KEY: u8 = 228;

pub const BTRFS_FT_REG_FILE: u8 = 1;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct BtrfsDevItem {
    /// the internal btrfs device id
    pub devid: u64,
    /// size of the device
    pub total_bytes: u64,
    /// bytes used
    pub bytes_used: u64,
    /// optimal io alignment for this device
    pub io_align: u32,
    /// optimal io width for this device
    pub io_width: u32,
    /// minimal io size for this device
    pub sector_size: u32,
    /// type and info about this device
    pub ty: u64,
    /// expected generation for this device
    pub generation: u64,
    /// starting byte of this partition on the device, to allow for stripe alignment in the future
    pub start_offset: u64,
    /// grouping information for allocation decisions
    pub dev_group: u32,
    /// seek speed 0-100 where 100 is fastest
    pub seek_speed: u8,
    /// bandwidth 0-100 where 100 is fastest
    pub bandwidth: u8,
    /// btrfs generated uuid for this device
    pub uuid: [u8; BTRFS_UUID_SIZE],
    /// uuid of FS who owns this device
    pub fsid: [u8; BTRFS_UUID_SIZE],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct BtrfsRootBackup {
    pub tree_root: u64,
    pub tree_root_gen: u64,
    pub chunk_root: u64,
    pub chunk_root_gen: u64,
    pub extent_root: u64,
    pub extent_root_gen: u64,
    pub fs_root: u64,
    pub fs_root_gen: u64,
    pub dev_root: u64,
    pub dev_root_gen: u64,
    pub csum_root: u64,
    pub csum_root_gen: u64,
    pub total_bytes: u64,
    pub bytes_used: u64,
    pub num_devices: u64,
    /// future
    pub unused_64: [u64; 4],
    pub tree_root_level: u8,
    pub chunk_root_level: u8,
    pub extent_root_level: u8,
    pub fs_root_level: u8,
    pub dev_root_level: u8,
    pub csum_root_level: u8,
    /// future and to align
    pub unused_8: [u8; 10],
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct BtrfsSuperblock {
    pub csum: [u8; BTRFS_CSUM_SIZE],
    pub fsid: [u8; BTRFS_FSID_SIZE],
    /// Physical address of this block
    pub bytenr: u64,
    pub flags: u64,
    pub magic: [u8; 0x8],
    pub generation: u64,
    /// Logical address of the root tree root
    pub root: u64,
    /// Logical address of the chunk tree root
    pub chunk_root: u64,
    /// Logical address of the log tree root
    pub log_root: u64,
    pub log_root_transid: u64,
    pub total_bytes: u64,
    pub bytes_used: u64,
    pub root_dir_objectid: u64,
    pub num_devices: u64,
    pub sector_size: u32,
    pub node_size: u32,
    /// Unused and must be equal to `nodesize`
    pub leafsize: u32,
    pub stripesize: u32,
    pub sys_chunk_array_size: u32,
    pub chunk_root_generation: u64,
    pub compat_flags: u64,
    pub compat_ro_flags: u64,
    pub incompat_flags: u64,
    pub csum_type: u16,
    pub root_level: u8,
    pub chunk_root_level: u8,
    pub log_root_level: u8,
    pub dev_item: BtrfsDevItem,
    pub label: [u8; BTRFS_LABEL_SIZE],
    pub cache_generation: u64,
    pub uuid_tree_generation: u64,
    pub metadata_uuid: [u8; BTRFS_FSID_SIZE],
    /// Future expansion
    pub _reserved: [u64; 28],
    pub sys_chunk_array: [u8; BTRFS_SYSTEM_CHUNK_ARRAY_SIZE],
    pub root_backups: [BtrfsRootBackup; 4],
}

fn map_to_file(filename: &str) -> Result<Mmap, &'static str> {
    let file = match File::open(filename) {
        Err(_) => {
            return Err("opening file");
        }
        Ok(f) => f,
    };
    return unsafe {
        match MmapOptions::new().map(&file) {
            Err(_) => {
                return Err("mmap error");
            }
            Ok(f) => Ok(f),
        }
    };
}

fn parse_block(memmapd: &Mmap) -> Vec<&[u8]> {
    // find superblock
    let mut superblocks = Vec::<&[u8]>::new();
    for i in (0..memmapd.len()).step_by(8) {
        match memmapd.get(i..i + 8) {
            Some(v) => {
                if v == BTRFS_SUPERBLOCK_MAGIC {
                    // write the superblock to the file
                    println!("found magic btrfs header at {:#01x}", i);
                    let start = i - 40;
                    match memmapd.get(start..start + 0xdcb) {
                        Some(block) => {
                            superblocks.push(block);
                        }
                        None => {
                            println!("Error finding superblock");
                            continue;
                        }
                    };
                }
            }
            None => {
                continue;
            }
        }
    }
    superblocks
}

/// Coverage map with explicit assignments due to the lack of instrumentation

fn main() -> Result<(), &'static str> {
    let filename = args().nth(1).expect("Usage: ./fuzzer [filesystem image]");
    let memmapd = match map_to_file(&filename) {
        Err(s) => {
            return Err(s);
        }
        Ok(f) => f,
    };

    let mut state = StdState::new(
        // RNG
        StdRand::with_seed(current_nanos()),
        // corpus that will be evolved, in memory for performance
        InMemoryCorpus::<BytesInput>::new(),
        // Corpus to store crashes
        OnDiskCorpus::<BytesInput>::new(PathBuf::from("./crashes")).unwrap(),
        (),
    );

    // copy the disk image
    let mut new_filename: String = filename.to_owned();
    new_filename.push_str("-mutated");
    let _ = match fs::copy(filename, new_filename) {
        Ok(v) => v,
        Err(_) => {
            return Err("Error copying disk image");
        }
    };

    // extract superblock
    let superblocks = parse_block(&memmapd);
    // fuzz the extracted block
    let mut mutator = StdScheduledMutator::new(havoc_mutations());
    let mut bytes_to_mutate = Vec::<u8>::new();
    for block in superblocks {
        for b in block {
            bytes_to_mutate.push(*b);
        }
        let mut input = BytesInput::new(bytes_to_mutate.clone());
        match mutator.mutate(&mut state, &mut input, -1) {
            Ok(_) => {
                println!("{:?}", input);
                continue;
            }
            Err(x) => {
                println!("Error mutating {:?}", x);
            }
        }
    }
    // generate a new checksum for each inserted superblock
    // place the mutated blocks into the copied disk image
    // mount the disk image
    // do file system operations on the disk image
    Ok(())
}
