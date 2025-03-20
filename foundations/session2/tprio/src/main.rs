use std::sync::atomic::{AtomicI32, Ordering};
use thread_priority::*;

static LOW_COUNT: AtomicI32 = AtomicI32::new(0);
static MEDIUM_COUNT: AtomicI32 = AtomicI32::new(0);
static HIGH_COUNT: AtomicI32 = AtomicI32::new(0);

fn low_prio() {
    set_current_thread_priority(ThreadPriority::Min).unwrap();
    loop {
        LOW_COUNT.fetch_add(1, Ordering::Relaxed);
        std::thread::yield_now();
    }
}

fn medium_prio() {
    loop {
        MEDIUM_COUNT.fetch_add(1, Ordering::Relaxed);
        std::thread::yield_now();
    }
}

fn main() {
    std::thread::spawn(low_prio);
    std::thread::spawn(medium_prio);
    std::thread::sleep(std::time::Duration::from_secs(5));
    println!("Low priority thread count: {}", LOW_COUNT.load(Ordering::Relaxed));
    println!("Medium priority thread count: {}", MEDIUM_COUNT.load(Ordering::Relaxed));
}
