use libafl::{
    bolts::{current_nanos, rands::StdRand},
    corpus::{InMemoryCorpus, OnDiskCorpus},
    feedbacks::{CrashFeedback, MapFeedbackState, MaxMapFeedback},
    fuzzer::StdFuzzer,
    inputs::BytesInput,
    mutators::{havoc_mutations, Mutator, StdScheduledMutator},
    observers::StdMapObserver,
    state::StdState,
};
use std::env::args;
use std::path::PathBuf;
use btrfs_parse;

fn main() -> Result<(), &'static str> {
    let filename = args().nth(1).expect("Usage: ./fuzzer [filesystem image]");
    println!("{:?}", btrfs_parse::btrfs_parse::BTRFS_SUPERBLOCK_MAGIC);
    /*let mut corpus = InMemoryCorpus::<BytesInput>::new();

    let mut state = StdState::new(
        // RNG
        StdRand::with_seed(current_nanos()),
        // corpus that will be evolved, in memory for performance
        corpus,
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
    // let superblocks = parse_block(&memmapd);
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
    // generate a new checksum for each inserted superblock
    // place the mutated blocks into the copied disk image
    // mount the disk image
    // do file system operations on the disk image
    */
    Ok(())
}
