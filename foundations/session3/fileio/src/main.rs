use std::{io::{self, BufRead}, path::Path, fs::File};

// Taken from: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// read_lines reads a file line by line and returns an iterator over the Lines
// The iterator has a lifetime bound to the lifetime of the file
// The file is wrapped in a BufReader to improve read performance
// The BufReader is then wrapped in a Lines iterator to simplify reading Lines
// The Lines iterator is returned
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/*
// synchronous main function
fn main() {
    let now = std::time::Instant::now(); // Start the timer
    let mut line_count = 0;
    if let Ok(lines) = read_lines("./war_and_peace.txt") { // Open the file and get 
                                                                                             // the iterator over the 
                                                                                             // lines in the file 
        lines.for_each(|line| { // count each line ignoring empty lines.
                                                       // Stop counting if there is an error
            if let Ok(line) = line{
                if !line.trim().is_empty() {
                    line_count += 1;
                }
            } 
        });
    }
    println!("Read {} lines in {:.3}", line_count, now.elapsed().as_secs_f64()); // Print the
                                                                                 // number of lines
                                                                                 // and the time
                                                                                 // taken
}
*/

// asynchronous main function but synchronous line_count function
async fn line_count(filename: String) -> io::Result<usize> {
    let now = std::time::Instant::now();
    let mut line_count = 0;
    if let Ok(lines) = read_lines(filename) {
        lines.for_each(|line| {
            if let Ok(line) = line {
                if !line.trim().is_empty() {
                    line_count += 1;
                }
            }
        });
    }
    println!("Read {} lines in {:.3}", line_count, now.elapsed().as_secs_f64());
    Ok(line_count)
}

// asynchronous main function and asynchronous line_count function
async fn async_line_count(filename: String) -> anyhow::Result<usize>{
    use tokio::io::AsyncBufReadExt; // Import the AsyncBufReadExt trait to add the next_line method
    use tokio::io::BufReader; // Import the BufReader struct to read the file asynchronously
    use tokio::fs::File; // Import the File struct to open the file asynchronously 
    println!("Reading {filename}...");
    let now = std::time::Instant::now();
    let mut line_count = 0;

    let file = File::open(filename).await?; // Open the file asynchronously
    let reader = BufReader::new(file); // Wrap the file in a BufReader
    let mut lines = reader.lines(); // Get an iterator over the lines in
                                                            // the file. Stream of Result<String>

    while let Some(line) = lines.next_line().await? { // Read the next line asynchronously
        if !line.trim().is_empty() {
            line_count += 1;
        }
    }

    println!("Read {} lines in {:.3}", line_count, now.elapsed().as_secs_f64());
    Ok(line_count)
}

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let filename = "./war_and_peace.txt";
    println!("Reading {filename}...");
    let now = std::time::Instant::now(); // Start the timer
    let (c1, c2) = tokio::join!(
        //line_count("./war_and_peace.txt".to_string()),
        //line_count("./war_and_peace.txt".to_string())
        async_line_count("./war_and_peace.txt".to_string()),
        async_line_count("./war_and_peace.txt".to_string())
        );
    println!("Total lines: {}", c1? + c2?);
    println!("In {:.3} seconds", now.elapsed().as_secs_f64());
    Ok(())
}
