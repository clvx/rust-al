use futures::executor::block_on;
use futures::join;
use futures::future::join_all;

fn do_something_sync() {
    println!("Doing something synchronously");
}

// say_hello is an async function that calls other async functions
async fn say_hello() {
    println!("Hello");
    join!(second_function(), say_goodbye());

    let n: u32 = double(4).await; // await is necessary to get the value from the future
    println!("{n}");

    let futures= vec![double(1), double(2), double(3)]; // create a
                                                                                      // vector of
                                                                                      // futures
    let results = join_all(futures).await; // join_all is used to wait for all
                                                           // futures to complete
    println!("{:?}", results);

    do_something_sync();
}

async fn second_function() {
    println!("Hello again!");
}

async fn say_goodbye() {
    println!("Goodbye!");
}

async fn double(n: u32) -> u32 {
    n * 2
}


fn main() {
    block_on(say_hello()); // block_on is used to run the async function
}
