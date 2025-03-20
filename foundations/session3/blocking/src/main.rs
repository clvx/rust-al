//use std::time::Duration;
use tokio::time::Duration;
use tokio::task::spawn_blocking;

async fn delay(task: u64, time: u64) {
    println!("Task {} has started", task);
    //std::thread::sleep(Duration::from_millis(time)); // sleeps the os thread
    tokio::time::sleep(Duration::from_millis(time)).await;  // sleeps the tokio runtime
    println!("Task {} has finished", task);
}

async fn blocking(task: u64, time: u64) {
    println!("Task {} has started", task);
    //spawn_blocking is used to run blocking code in a non-blocking way, 
    //it will spawn a new thread to run the blocking code
    let _ = spawn_blocking(move|| {
        std::thread::sleep(Duration::from_millis(time));
    }).await;
    println!("Task {} has finished", task);
}

#[tokio::main]
async fn main() {
    tokio::join!(
        delay(1, 500), 
        delay(2, 1000),
        delay(3, 1500),
        );
    tokio::join!(
        blocking(4, 500), 
        blocking(5, 1000),
        blocking(6, 1500),
        );


}
