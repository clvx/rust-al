use std::sync::{Arc, Mutex};

struct SharedData(String);

fn main() {
    // Create a shared data structure and wrap it in a mutex and an atomic reference count
    // using Arc and Mutex respectively to share it between threads safely and efficiently
    //
    // External mutability is required to modify the shared data structure in multiple threads
    // so we wrap it in a Mutex to ensure that only one thread can access the data at a time
    // and we wrap the Mutex in an Arc to share the data between threads and increment thread
    // reference count.
    let my_shared = Arc::new(Mutex::new(SharedData("Hello".to_string())));
    println!(" Mut Atomic Reference count {}", Arc::strong_count(&my_shared));
    let mut threads = Vec::new(); // Create a vector to hold the threads
    for i in 0..10 {
        let my_shared = my_shared.clone(); // Increment the reference count
                                                                   // in i
        println!(" Mut Atomic Reference count before spawning threads {}", Arc::strong_count(&my_shared));
        threads.push(std::thread::spawn(move || {
            let mut data = my_shared.lock().unwrap(); // Lock the mutex
                                                                                  // to access the
                                                                                  // shared
                                                                                  // structure
            data.0.push_str(&format!(" {i}")); // push the value of i to the shared
                                                       // structure. push_str is used to append 
                                                       // a string to the shared data structure
        })); // The mutex is unlocked when data goes out of scope and the atomic 
             // reference count is decremented
    }
    for t in threads {
        t.join().unwrap();
    }
    println!(" Mut Atomic Reference count after the threads {}", Arc::strong_count(&my_shared));
    let data = my_shared.lock().unwrap();
    println!("{}", data.0); // Print the shared data structure after all threads
                                    // have modified it
}
