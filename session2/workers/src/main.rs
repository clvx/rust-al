use std::sync::mpsc;

type Job = Box<dyn FnOnce() + Send + 'static>;

fn hi_there() {
    println!("Hi there!");
}

enum Command {
    Run(Job),
    Stop,
}

fn main() {
    //let (tx, rx) = mpsc::channel::<Job>();
    let (tx, rx) = mpsc::channel::<Command>();
    let handle = std::thread::spawn(move || {
        //while let Ok(job) = rx.recv() {
        while let Ok(command) = rx.recv() {
            match command {
                Command::Run(job) => job(),
                Command::Stop => break,
            }
            //job();
        }
    });

    let job = || println!("Hello from closure");
    let job2 = || {
        for i in 0..10 {
            println!("Counting: {}", i);
        }
    };
    
    /*
    tx.send(Box::new(job)).unwrap();
    tx.send(Box::new(job2)).unwrap();
    tx.send(Box::new(hi_there)).unwrap();
    tx.send(Box::new(||println!("I'm in a box!"))).unwrap();
    */
    tx.send(Command::Run(Box::new(job))).unwrap();
    tx.send(Command::Run(Box::new(job2))).unwrap();
    tx.send(Command::Run(Box::new(hi_there))).unwrap();
    tx.send(Command::Stop).unwrap();

    handle.join().unwrap();
}
