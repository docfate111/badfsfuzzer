use fs_parse::btrfs_parse::*;
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

fn main() -> Result<(), &'static str> {
    let filename = args().nth(1).expect("Usage: ./fuzzer [filesystem image]");
    let original = filename.clone();
    let mut new_file: String = filename.to_owned();
    new_file.push_str("-metadata");
    // extract metadata and create a copy to put metadata into
    match extract(&original, &new_file) {
        Ok(_) => {}
        Err(_) => {
            return Err("Error extracting superblock");
        }
    }
    let mut metadata_file = String::from("./corpus/"); 
    metadata_file.push_str(&new_file);
    // read metadata into bytes
    let bytes_to_mutate =
        read_into_vec(&metadata_file).expect("error mmaping within read_into_vec");
    let mut state = StdState::new(
        // RNG
        StdRand::with_seed(current_nanos()),
        // Corpus that will be evolved, in memory for performance
        OnDiskCorpus::<BytesInput>::new(PathBuf::from("./corpus")).unwrap(),
        // Corpus to store crashes
        OnDiskCorpus::<BytesInput>::new(PathBuf::from("./crashes")).unwrap(),
        (),
    );
    // fuzz the extracted block
    //println!("{:?}", bytes_to_mutate);
    let mut mutator = StdScheduledMutator::new(havoc_mutations());
    let mut input = BytesInput::new(bytes_to_mutate.clone());
    //for _ in 0..0 {
    match mutator.mutate(&mut state, &mut input, -1) {
        Ok(_) => {
            println!("{:?}", input);
        },
        Err(x) => {
            println!("Error mutating {:?}", x);
        }
    }
    //}
    // generate a new checksum for each inserted superblock
    // place the mutated blocks into the copied disk image
    // mount the disk image
    // do file system operations on the disk image
    Ok(())
}
