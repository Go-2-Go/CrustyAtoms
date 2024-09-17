use core::panic;
use std::{
    fs::File,
    error::Error, 
    io::{self, BufRead},
    path::Path,
    env,
    collections::HashMap};

const TRIGGER_CHANNEL: usize = 7;
const NUMBER_OF_CHANNELS: usize = 6;

pub fn sieve(
    filename: &str,
    trigger_tolerance: usize) -> [Option<Vec<usize>>; NUMBER_OF_CHANNELS] 
{
    let data_collected: [Option<Vec<usize>>; NUMBER_OF_CHANNELS] = [const { None }; NUMBER_OF_CHANNELS];
    
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            println!("{}", line);
        }
    }

    data_collected

}

//fn process_file(contents: Vec<u8>) -> &str

/// The output is wrapped in a Result to allow matching on errors.
/// Returns an Iterator to the Reader of the lines of the file.
/// Ref: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
