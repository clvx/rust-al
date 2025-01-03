fn single_hello_tread() {
    println!("Hello from the single thread");
}



fn main() {
    println!("Hello from the main thread");

    let single_thread_handle = std::thread::spawn(single_hello_tread);
    single_thread_handle.join().unwrap();
}
