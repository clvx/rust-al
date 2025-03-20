use std::sync::Mutex;

// needed to make the Mutex static and available in all scopes
static MY_SHARED: Mutex<i32> = Mutex::new(1);

fn posioner() {
    let mut lock = MY_SHARED.lock().unwrap();
    *lock += 1;
    panic!("got poisoned");
}


fn main() {
    // -- DEADLOCK --

    //without the scope the lock is not released
    {
        let _lock = MY_SHARED.lock().unwrap();
    }
    let _lock = MY_SHARED.lock().unwrap();
    //when commentted the lock is locked
    std::mem::drop(_lock);
    if let Ok(_lock) = MY_SHARED.try_lock() {
        println!("I got the lock");
    } else {
        println!("I did not get the lock");
    }
    
    // -- POISONING --
    let handle = std::thread::spawn(|| {
        posioner();
    });
    println!("Trying to return from thread");
    println!("{:?}", handle.join());

    let lock = MY_SHARED.lock();
    println!("{lock:?}");

    let recovered_data = lock.unwrap_or_else(|poisoned| {
        println!("Mutex was poisoned, recovering data...");
        poisoned.into_inner() //returns the data inside the Mutex before poisoning
    });
    println!("Recovered data: {}", recovered_data); //prints 2
}
