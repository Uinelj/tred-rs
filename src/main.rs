extern crate rand;

use std::thread;
use std::time::Duration;

use rand::distributions::{IndependentSample, Range};
use rand::Rng;

fn main() {
    let nthread = 4;
    let mut rng = rand::thread_rng();
    let rng_boundaries = Range::new(1u64, 10u64);
    let mut secs;
    let mut handles = vec![];
    for i in 0..nthread {
        secs = rng_boundaries.ind_sample(&mut rng);
        handles.push(thread::spawn(move || {
            println!("Thread number {} started. Will work for {} seconds", i, secs);
            work(secs);
            println!("Thread number {} ended", i);
        }))
    }

    for handle in handles {
        let _ = handle.join();
    }
}

fn work(secs : u64) {
    thread::sleep(Duration::from_millis(secs * 1000));
}
