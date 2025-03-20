fn single_hello_tread() {
    println!("Hello from the single thread");
}

fn hello_tread(i: i32) {
    println!("Hello from the thread {}", i);
}

fn do_math(i: u32) -> u32 {
    let mut n = i+1;
    for _ in 0..10 {
        n = n * 2;
    }
    n
}

fn main() {
    println!("Hello from the main thread");

    let single_thread_handle = std::thread::spawn(single_hello_tread);
    single_thread_handle.join().unwrap();

    let mut thread_handles = Vec::new();
    for i in 0..5 { 
        //The move keyword captures the value of i by value. This ensures that 
        // the thread gets its own copy of i rather than borrowing it.
        //Without move, the closure would borrow i, leading to potential issues 
        // as the loop modifies it.
        let thread_handle = std::thread::spawn(move || hello_tread(i));
        thread_handles.push(thread_handle);
    }
    /*
    into_iter() method consumes the return_thread_handles vector, turning it into an iterator.
    join() method is called on each JoinHandle to wait for the associated thread to complete its execution.
    unwrap() ensures the program panics if a thread fails (e.g., due to a panic inside the thread).
    thread_handles.into_iter().for_each(|handle| handle.join().unwrap());
    */
    let mut return_thread_handles = Vec::new();
    for i in 0..5 { 
        let thread_handle = std::thread::spawn(move || do_math(i));
        return_thread_handles.push(thread_handle);
    }
    return_thread_handles.into_iter().for_each(|handle| 
        { println!("{}", handle.join().unwrap());
        });

}
