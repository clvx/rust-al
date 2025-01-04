//stati mut COUNTER: i32 = 0;

use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;
use std::thread;

static COUNTER: AtomicI32 = AtomicI32::new(0);

fn main() {
    let mut handles = Vec::new();
    for _ in 0..1000 {
        let handle = thread::spawn(|| {
            for _ in 0..1_100 {
                //unsafe{
                //    COUNTER += 1;
                //}
                // atomically increments the counter by 1 without requiring a lock.
                // Ordering::Relaxed - No ordering guarantees; fastest option.
                COUNTER.fetch_add(1, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }
    handles.into_iter().for_each(|h| h.join().unwrap());
    //unsafe{
    //    println!("COUNTER: {}", COUNTER);
    //};
    println!("COUNTER: {}", COUNTER.load(Ordering::Relaxed)); // Read atomically
}
