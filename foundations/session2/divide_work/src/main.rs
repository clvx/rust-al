fn main() {
    const N_THREADS : usize = 8;
    let to_add: Vec<u32> = (0..5000).collect();
    let mut thread_handles = Vec::new();
    //Divides the to_add vector into smaller slices of size N_THREADS
    let chunks = to_add.chunks(N_THREADS);

    for chunk in chunks {
        //Converts the chunk (a slice) into an owned Vec<u32> to ensure that the 
        //data is moved into the thread safely.
        let my_chunk: Vec<u32> = chunk.to_owned();
        //The move keyword ensures the my_chunk vector is moved into the thread's 
        //closure, giving the thread exclusive ownership of the data.
        thread_handles.push(std::thread::spawn(move || {
            //Inside the thread, my_chunk.iter().sum::<u32>() calculates the sum 
            //of the numbers in the chunk.
            my_chunk.iter().sum::<u32>()
        }));
    }

    // Total of each chunk's sum
    let mut sum = 0;
    for handle in thread_handles {
        // join() waits for each thread to finish and retrieves the sum 
        // of the corresponding chunk.
        // unwrap() ensures the program panics if a thread fails.
        sum += handle.join().unwrap();
    }
    println!("Sum: {}", sum);

}
