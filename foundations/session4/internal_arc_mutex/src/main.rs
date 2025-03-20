use std::sync::{Arc, Mutex};

struct SharedData{
    data: Mutex<String>, // Wrap the data in a mutex to ensure that only one thread can access the
                         // data at a time
}

impl SharedData {
    fn new(s: &str) -> Self {
        Self {
            data: Mutex::new(s.to_string()), // Initialize the data field with a Mutex
                                             // containing the string shared between threads
                                             // and wrapped in a Mutex
        }
    }
}

fn main() {
    // interior mutability is required to modify the shared data structure in multiple threads
    // so we wrap it in a Mutex to ensure that only one thread can access the data at a time
    // and we wrap the Mutex in an Arc to share the data between threads and increment threads
    // reference count.
    let my_shared = Arc::new(SharedData::new("Hello"));
    println!(" Mut Atomic Reference count {}", Arc::strong_count(&my_shared));
    let mut threads = Vec::new();
    for i in 0..10 {
        let my_shared = my_shared.clone();
        println!(" Mut Atomic Reference count before spawning threads {}", Arc::strong_count(&my_shared));
        threads.push(std::thread::spawn(move || {
            let mut data = my_shared.data.lock().unwrap(); // Lock the mutex to access the shared
                                                           // structure
            data.push_str(&format!(" {i}")); // push the value of i to the shared structure
                                                     // push_str is used to append a string to the
                                                     // shared data structure.
        })); // The mutex is unlocked when data goes out of scope and the atomic reference count is
             // decremented
    }
    for t in threads {
        t.join().unwrap();
    }
    println!(" Mut Atomic Reference count after the threads {}", Arc::strong_count(&my_shared));
    let data = my_shared.data.lock().unwrap();
    println!("{data}"); // print the shared data structure after all threads have modified it
}
