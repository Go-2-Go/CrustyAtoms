use std::{error, fmt};
use itertools::izip;
use log::{trace, debug};

use super::extractor::*;

#[derive(Debug, Clone)]
struct TimeSumError {
    message: String,
}
impl fmt::Display for TimeSumError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error while calculating timesum for data:\n {}", &self.message)
    }
}


impl error::Error for TimeSumError {}

/// Extract the TimeSum for given channel data.
/// Uses the MCP data (mcp) to locate closest hit on both 
/// DLD channel ends (x1 and x2) and looks for pairs of hits 
/// for which 0 <= x1 + x2 - 2 * mcp <= TIMESUM_UPPER.
pub fn timesum_extractor(
    mcp_data: &Vec<usize>,
    end_1_data: &Vec<usize>, 
    end_2_data: &Vec<usize>,
    tsum_tol: usize,
    ) -> Result<Vec<isize>, Box<dyn error::Error>> {

    // Vectors for holding closest hits on the dld channel
    // ends for each mcp hit.
    let mut end_1_indices: Vec<usize> = Vec::new();
    let mut end_2_indices: Vec<usize> = Vec::new();

    // Vector for holding timesum values.
    let mut timesums: Vec<isize> = Vec::new();

    // Populate the indices arrays
    for mcp_hit in mcp_data.iter() {
        // Search for indices in end 1 data from where to start searching for this mcp_hit.
        end_1_indices.push(binary_search_position(end_1_data, &(mcp_hit - tsum_tol)));
        // Search for indices in end 2 data from where to start searching for this mcp_hit.
        end_2_indices.push(binary_search_position(end_2_data, &(mcp_hit - tsum_tol)));
    }

    // Populate the timesum array using nested loops.
    for (mcp_hit, end_1_start, end_2_start) in izip!(mcp_data.iter(), end_1_indices.iter(), end_2_indices.iter()) {
        let mut end_1_focus = *end_1_start;
        let mut end_2_focus = *end_2_start;
        while end_1_focus < end_1_data.len() && end_1_data[end_1_focus] < mcp_hit + TIME_SUM_UPPER {
            while end_2_focus < end_2_data.len() && end_2_data[end_2_focus] < mcp_hit + TIME_SUM_UPPER {
                timesums.push((end_1_data[end_1_focus] + end_2_data[end_2_focus]) as isize - (2 * mcp_hit) as isize);
                end_2_focus += 1;
            }
            end_2_focus = *end_2_start;
            end_1_focus += 1;
        }
    }
    
    Ok(timesums)
}
