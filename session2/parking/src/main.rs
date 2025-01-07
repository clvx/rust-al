fn parkable_thread(n: u32){
    loop {
        std::thread::park();
        println!("Thread {} has been parked", n);
    }  
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut threads = Vec::new();
    for i in 0..10 {
        let thread = std::thread::spawn(move || {
            parkable_thread(i);
        });
        threads.push(thread);
    }

    loop {
        println!("Thread to unpark (q to quit): ");
        let input = read_line();
        if input == "q" {
            break;
        }
        //Attempts to convert the input (a string) into a usize. If the parsing 
        // is successful, the number is extracted into the variable number.
        // If parsing fails (e.g., the input is not a valid integer), the if let block is skipped.
        if let Ok(number) = input.parse::<usize>() {
            if number < threads.len() {
                threads[number].thread().unpark();
            }
        }
    }
}
