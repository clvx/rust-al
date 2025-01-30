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
    while let Ok(command) = rx.recv() {
        let _ = sender::send_command(command);
    }
}
