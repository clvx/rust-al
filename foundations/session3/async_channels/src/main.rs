use std::{time::Duration, sync::mpsc};

enum Command {
    Print(String),
}

//main() sends a command to the command thread every second. The command thread will then 
// send a reply back to the main thread after 10 miliseconds. The main thread will then print the 
// reply. The command thread is a blocking thread that will not block the async runtime. 
// The reply channel is cloned and sent to the blocking task to be able to send the reply back
// to the async runtime. The blocking task is spawned using the runtime handle. 
// The main thread will print the reply received from the command thread. 
// The command thread will receive the command from the main thread and send the reply back to the
// main thread.
#[tokio::main]
async fn main() {
    // Spawn a command thread for "heavy lifting"
    let (tx, rx) = mpsc::channel(); // Create a channel for the
    let (tx_reply, mut rx_reply) = tokio::sync::mpsc::channel::<String>(10);  // Create
                                                                                                            // a
                                                                                                            // channel
                                                                                                            // for
                                                                                                            // the
                                                                                                            // reply
    let handle = tokio::runtime::Handle::current(); // Get the current runtime handle. This
                                                            // is needed to spawn a blocking task
                                                            // inside the async runtime.
    
    std::thread::spawn(move ||{
        while let Ok(command) = rx.recv() {
            match command {
                Command::Print(msg) => {
                    let tx_reply = tx_reply.clone(); // Clone the reply channel to be able to
                                                                     // send it to the blocking task
                    handle.spawn(async move { // Spawn a blocking task inside the async
                                                      // runtime
                                                      // This is a blocking task that will run in a
                                                      // separate thread pool and will not block
                                                      // the async runtime thread pool.
                        // Simulate a blocking task
                        tokio::time::sleep(Duration::from_millis(10)).await;
                        tx_reply.send(msg).await.unwrap(); // Send the reply back 
                                                           // to the async runtime
                    });
                }
            }
        }
    });

    // Receive messages
    tokio::spawn(async move { // Spawn an async task to receive messages from the
                                          // command thread and print them
        while let Some(reply) = rx_reply.recv().await {
            println!("{}", reply);
        }
    });

    // Launch the async sender
    let mut counter = 0;
    // This is the main loop that will run in the async runtime
    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
        tx.send(Command::Print(format!("Counter: {}", counter))).unwrap();
        counter += 1;
    }
}
