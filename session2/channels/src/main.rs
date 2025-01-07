use std::sync::mpsc::channel;

enum Command{
    SayHello, Quit
}
fn main() {
    let (tx, rx) = channel::<Command>();

    let handle = std::thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            match command {
                Command::SayHello => println!("Hello"),
                Command::Quit => {
                    println!("Quitting");
                    break;
            }
          }
        }
    });

    println!("Sending Hello");
    for _ in 0..10 {
        tx.send(Command::SayHello).unwrap();
    }
    std::thread::sleep(std::time::Duration::from_secs(1));

    println!("Sending Quit");
    tx.send(Command::Quit).unwrap();
    handle.join().unwrap();
}
