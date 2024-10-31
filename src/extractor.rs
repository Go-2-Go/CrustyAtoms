use log;
use core::panic;
use std::{collections::HashMap, error, fmt, usize};
use log::{info, warn, debug};
use itertools::izip;

#[derive(Debug, Clone)]
struct ExtractorError {
    message: String,
}
impl fmt::Display for ExtractorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error while extracting data:\n {}", &self.message)
    }
}
impl error::Error for ExtractorError {}

const HIT_TIME_TOLERANCE:usize = 0;
const TIME_SUM_UPPER: usize = 4000;

pub fn extractor(
    mcp_data: &Vec<usize>,
    end_1_data: &Vec<usize>, 
    end_2_data: &Vec<usize>,
    tsum: usize,
    tsum_tolerance: usize,
    ) -> Result<(Vec<isize>, Vec<bool>), Box<dyn error::Error>> {


    const EMPTY_VEC: Vec<usize> = Vec::new();

    let mut end_1_indices: [Vec<usize>; 2] = [EMPTY_VEC; 2];
    let mut end_2_indices: [Vec<usize>; 2] = [EMPTY_VEC; 2];

    // Find places in channel data where mcp hits would fit without disturbing the order.
    for mcp_hit in mcp_data.iter() {

        // Search for indices in end 1 data from where to start searching for this mcp_hit.
        end_1_indices[0].push(binary_search_position(end_1_data, &(mcp_hit - HIT_TIME_TOLERANCE)));
        // Search for indices in end 1 data from where to end searching for this mcp_hit.
        end_1_indices[1].push(binary_search_position(end_1_data, &(mcp_hit + TIME_SUM_UPPER)));

        // Search for indices in end 2 data from where to start searching for this mcp_hit.
        end_2_indices[0].push(binary_search_position(end_2_data, &(mcp_hit - HIT_TIME_TOLERANCE)));
        // Search for indices in end 2 data from where to end searching for this mcp_hit.
        end_2_indices[1].push(binary_search_position(end_2_data, &(mcp_hit + TIME_SUM_UPPER)));
    }

    let end_1_extracted = get_extracted_hits(&end_1_indices[0], &end_1_indices[1], end_1_data);
    let end_2_extracted = get_extracted_hits(&end_2_indices[0], &end_2_indices[1], end_2_data);
    
    let mut mask: Vec<bool> = mcp_data.iter().map(|_| false).collect();
    let mut reconstructed: Vec<isize> = mcp_data.iter().map(|_| 0).collect();
    for (end_1_hits, end_2_hits, &mcp_hit, reconstructed_hit, mask_val) in izip!(end_1_extracted.iter(), end_2_extracted.iter(), mcp_data.iter(), &mut reconstructed, &mut mask) {
        if end_1_hits.len() == 1 && end_2_hits.len() == 1 { 
            let end_1_hit = match end_1_hits[0].checked_sub(mcp_hit) {
                Some(k) => k,
                None => panic!("Hit {}, mcp hit {}", end_1_hits[0], mcp_hit),
            };
            let end_2_hit = match end_2_hits[0].checked_sub(mcp_hit) {
                Some(k) => k,
                None => panic!("Hit {}, mcp hit {}", end_2_hits[0], mcp_hit),
            };
            if (end_1_hit + end_2_hit).abs_diff(tsum) <= tsum_tolerance {
                *reconstructed_hit = end_1_hit as isize - end_2_hit as isize;
                *mask_val = true;
            }
        }
    }

    info!("Done extracting");

    return Ok((reconstructed, mask));
}

/// Using the start and end indices, return a vector of slices of data array.
fn get_extracted_hits<'a>(start_indices: &Vec<usize>, end_indices: &Vec<usize>, hit_data: &'a Vec<usize>) -> Vec<&'a [usize]> {

    let mut collected_hits: Vec<&[usize]> = Vec::new();
    // Use the start and end indices to retrieve hits on channels.
    for (&start_index, &end_index) in start_indices.iter().zip(end_indices.iter()) {
        // Take slice of data points with the start and end points.
        let data = &hit_data[start_index..end_index];
        collected_hits.push(data);
    }
    collected_hits
}

#[derive(Debug, Clone)]
struct ItemExceedsLastVal;
impl fmt::Display for ItemExceedsLastVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Item not found while searching.")
    }
}
impl error::Error for ItemExceedsLastVal {}

/// Returns the index 'i' such that data[i-1] <= item <= data[i].
/// If value exceeds last value in vector, then returns length of vector
fn binary_search_position <T: PartialOrd> (data: &Vec<T>, item: &T) -> usize {
    let mut left    = 0;
    let mut right   = data.len() - 1;
    let mut mid     = 0;
    if *item > data[right] { return right + 1 }
    if *item < data[left] { return 0 }
    let mut mid_val;
    while right > left + 1 {
        mid = (left + right) >> 1;
        mid_val = &data[mid];
        if mid_val <= item { left = mid; }
        else if item < mid_val { right = mid; }
    }
    return left + 1;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_binary_search() {
        let vals: Vec<usize> = vec!(10, 22, 33, 45);
        assert_eq!(binary_search_position(&vals, &4), 0);
        assert_eq!(binary_search_position(&vals, &23), 2);
        assert_eq!(binary_search_position(&vals, &13), 1);
        let vals: Vec<usize> = vec!(0, 2, 3, 5);
        assert_eq!(binary_search_position(&vals, &4), 3);
        assert_eq!(binary_search_position(&vals, &6), 4);
        assert_eq!(binary_search_position(&vals, &1), 1);
    }
}
