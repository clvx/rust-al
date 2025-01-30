use std::collections::VecDeque;

use shared_data::CollectorCommandV1;
mod data_collector;
mod sender;
mod errors;

fn main() {
    let (tx, rx) = std::sync::mpsc::channel::<CollectorCommandV1>();

    // Start the collector thread
    let _collector_thread = std::thread::spawn(move || {
        data_collector::collect_data(tx);
    });

    // Listen for commands to send
    let mut send_queue = VecDeque::with_capacity(120);
    while let Ok(command) = rx.recv() {
        let encoded = shared_data::encode_v1(&command);
        send_queue.push_back(encoded); // Add the command to the queue
        
        let _ = sender::send_queue(&mut send_queue);
        /*
        // send all queued commands in different connections
        while let Some(command) = send_queue.pop_front() {
            if sender::send_command(&command).is_err() {
                println!("Error sending command");
                send_queue.push_front(command); // Put the command back in the queue
                break;
            }   
        }
        */
    }
}
