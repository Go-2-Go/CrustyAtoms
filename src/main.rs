mod sieve;
use std::{env, process, thread};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Hello, world!");

    let mut list = vec![1, 2, 3];
    println!("Before closure {:?}", list);

    thread::spawn(move || 
        {list.push(7);
        println!("After closure {:?}", list);
    })
    .join()
    .unwrap();
    sieve::sieve("121023_btrap_17.txt", 10);

}
