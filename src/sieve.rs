use core::panic;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path};
use log::{info, warn};

use itertools::Itertools;

const TRIGGER_CHANNEL: usize = 7;
const NUMBER_OF_CHANNELS: usize = 7;

pub fn sieve(
    filename: &str,
    trigger_tolerance: usize) -> [Vec<usize>; NUMBER_OF_CHANNELS] 
{
    const EMPTY_VEC: Vec<usize> = Vec::new();
    let mut data_collected: [Vec<usize>; NUMBER_OF_CHANNELS] = [EMPTY_VEC; NUMBER_OF_CHANNELS];
    
    match read_lines(filename) {
        Ok(mut lines) => {

            let trigger: usize = get_trigger(&mut lines);

            // Skip remaining trigger events and other events which are not
            // within tolerance.
            skip_false_counts(&mut lines, trigger, trigger_tolerance);

            while let Some(Ok(line)) = lines.next() {

                let (channel, event_time) = process_line(line);

                let event_time = event_time - trigger;

                data_collected[channel].push(event_time);
            }

            info!("Done reading");
        },
        Err(_) => panic!("Not found file")
    }

    for channel in 0..7 { 
        let sorted = is_sorted(&data_collected[channel]);
        if !sorted {
            warn!("Channel {channel} is not sorted, sorting ...");
            data_collected[channel].sort();
        } else { info!("Channel {channel} is sorted"); }
    }
    data_collected

}

/// Checks if a vector is sorted.
fn is_sorted <T: PartialOrd> (data: &Vec<T>) -> bool {
    for index in 0..(data.len() - 1) {
        if data[index] > data[index + 1] {return false}
    }
    return true
}

fn process_line (line: String) -> (usize, usize) {
    match line.split(',').next_tuple() {
        Some((channel, event_time)) => return (channel.parse().unwrap(), event_time.parse().unwrap()),
        None => panic!("Unexpected data file format.")
    };
}

fn skip_false_counts (lines: &mut io::Lines<io::BufReader<File>>, trigger: usize, trigger_tolerance: usize) {
    // Start processing the remaining data file.
    while let Some(Ok(line)) = lines.next() {
        // Parses a line in the data file
        let (channel, event_time) = process_line(line);

        // Ignore trigger events which come after the first and events which are not within
        // trigger tolerance of the trigger.
        if (channel != 7) && (event_time - trigger) < trigger_tolerance { return; }
    }
}

fn get_trigger (lines: &mut io::Lines<io::BufReader<File>>) -> usize {

    // Get the trigger which is the first entry in data file
    let trigger_line = lines.next().unwrap().unwrap();
    let trigger_line: Vec<&str> = trigger_line.split(',').collect();

    // Check if the first value is on trigger channel and set the trigger
    // Panics if first entry is not on trigger channel
    if trigger_line[0].parse() == Ok(TRIGGER_CHANNEL) {
        return trigger_line[1].parse().unwrap()
    } else {
        panic!("First val not trigger")
    };
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
