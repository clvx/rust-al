//use std::sync::Mutex;
use tokio::sync::Mutex;
use once_cell::sync::Lazy;

//static COUNTER: Mutex<u32> = Mutex::new(0);
static COUNTER: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));

async fn add_one(n: u32) -> u32 {
    n + 1
}

async fn increment() {
    //let mut counter = COUNTER.lock().unwrap();
    let mut counter = COUNTER.lock().await; //The line COUNTER.lock().await 
                                                                 //acquires the asynchronous mutex. 
                                                                 //If another task is holding the lock, 
                                                                 //the current task is suspended until 
                                                                 //the lock becomes available.
    *counter = add_one(*counter).await; //Once the lock is acquired, the current value of the counter 
                                        //is incremented using add_one (which is awaited) and the 
                                        //result is written back
}

#[tokio::main]
async fn main() {
    tokio::join!(increment(), increment(), increment()); // The tokio::join! macro is used to run 
                                                          // multiple asynchronous functions concurrently. 
                                                          // In this case, increment is called three times 
                                                          // concurrently, which means that the counter 
                                                          // is incremented three times in parallel.
    //println!("COUNTER = {}", *COUNTER.lock().unwrap());
    println!("COUNTER = {}", *COUNTER.lock().await); //The lock is acquired again to read the final
                                                     //value of the counter.
}
