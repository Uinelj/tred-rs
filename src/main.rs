extern crate rand;

use std::thread;
use std::time::Duration;

use rand::distributions::{IndependentSample, Range};
use rand::Rng;

fn main() {

    let nthread = 4;
    let mut rng = rand::thread_rng();
    let rng_boundaries = Range::new(1u64, 4u64);
    let mut secs;
    let mut handles = vec![];

    for i in 0..nthread {
        secs = rng_boundaries.ind_sample(&mut rng);
        handles.push(thread::spawn(move || {
            println!("{} : Thread started. Estimated work time : {} ", i, secs);
            println!("{} : Parking thread", i);
            thread::park();
            println!("{} : Thread unparked ! Working...", i);
            work(secs);
            println!("{} : Thread ended", i);
        }));


    }

    loop {
        for handle in handles.iter().cycle() {
            //println!("M : Unparking thread");
            &handle.thread().unpark();
            thread::sleep(Duration::from_millis(10));
        }
}
}

fn work(secs : u64) {
    thread::sleep(Duration::from_millis(secs * 1000));
}
