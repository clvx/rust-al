use std::thread;

fn simple_thread() {
    println!("hello from a thread named {}", thread::current().name().unwrap());
}

fn my_thread(i: i32) {
    println!("hello from a thread named {}, processing task {}",
        thread::current().name().unwrap(), 
        i
    );
}

fn main() {
    
    //single thread
    thread::Builder::new()
        .name("single_thread".to_string())
        .stack_size(std::mem::size_of::<usize>() * 4)
        .spawn(simple_thread)
        .unwrap()
        .join()
        .unwrap();
    

    //running multiple threads
    let mut handles = Vec::new();

    for i in 0..5 {
        let builder = thread::Builder::new()
            .name(format!("Worker-{}", i))
            .stack_size(std::mem::size_of::<usize>() * 4);
        let handle = builder.spawn(move || my_thread (i) ).unwrap();

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

