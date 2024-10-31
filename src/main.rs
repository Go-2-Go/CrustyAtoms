mod sieve;
mod extractor;
use std::{collections::HashMap, env, error, thread};

use itertools::izip;
use log::{debug, info, trace};

fn main() -> Result<(), Box<dyn error::Error>>{
    env_logger::init();
    info!("Starting");

    let _args: Vec<String> = env::args().collect();
    println!("Hello, world!");

    let mut list = vec![1, 2, 3];
    println!("Before closure {:?}", list);

    thread::spawn(move || 
        {list.push(7);
        println!("After closure {:?}", list);
    })
    .join()
    .unwrap();

    let data = sieve::sieve("test_files/121023_btrap_17.txt", 10_usize.pow(8));
    let mut i = 0;
    data.iter().for_each(|x| {
        info!("Channel {} size {}", i, x.len());
        i+=1;
    });

    info!("Extracting indices from channels X1 and X2");
    let (x_reconstructed, x_mask) = extractor::extractor(&data[0],
                                                         &data[1],
                                                         &data[2],
                                                         4000,
                                                         1000)?;
    let mut counter: usize = 0;
    for (hit, mask) in izip!(x_reconstructed, x_mask){
        if mask { 
            trace!("{}", hit);
            counter += 1; 
        }
    }
    debug!("{}", counter);
    
    info!("Extracting indices from channels Y1 and Y2");
    let (y_reconstructed, y_mask) = extractor::extractor(&data[0],
                                                         &data[3],
                                                         &data[4],
                                                         4000,
                                                         1000)?;

    info!("Extracting indices from channels Z1 and Z2");
    let (z_reconstructed, z_mask) = extractor::extractor(&data[0],
                                                         &data[4],
                                                         &data[5],
                                                         4000,
                                                         1000)?;
    //println!("Channel {}, at time {}s", channel, (event_time * 25) as f32 * 10.0_f32.powf(-12.0));
    
    Ok(())
}
