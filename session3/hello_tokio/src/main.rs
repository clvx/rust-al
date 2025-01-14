//use tokio::runtime;
use tokio::time::{sleep, Duration};

async fn hello(s: &str) {
    println!("Hello, {}!", s);
}

async fn hola(s: &str) -> String {
    format!("Hola, {}!", s)
}

async fn ciao(s: &str) -> String {
    format!("Ciao, {}!", s)
}

async fn ticker() {
    for i in 0..10 {
        println!("tick {}", i);
        sleep(Duration::from_secs(1)).await;
        tokio::task::yield_now().await; //Yield execution to the Tokio runtime.
    }
}

async fn tocker() {
    for i in 0..10 {
        println!("tock {}", i);
        sleep(Duration::from_secs(2)).await;
        tokio::task::yield_now().await; //Yield execution to the Tokio runtime.
    }
}

// using tokio macro
//#[tokio::main(flavor = "current_thread")]  //when running in single-threaded mode, the program runs
                                           //slowly but still concurrently.
#[tokio::main] //when running in multi-threaded mode, the program runs quickly, concurrently, and
              // probably parallelly.
async fn main() {


    hello("world").await; //Await a future.
    hello("multi world").await; //Await a future.

    let result = tokio::join!(hola("mundo"), ciao("mondo")); //Join multiple
                                                                               //futures together,
                                                                               //waiting for all of
                                                                               //them to complete.
    println!("{:?}", result);
    let (saludos, saluti) = result;
    println!("{}, {}", saludos, saluti);

    _ = tokio::join!(
        tokio::spawn(ticker()),
        tokio::spawn(tocker()),
        ); //Spawn a future onto the Tokio runtime and ignore its result.
}

/*
fn main() {
    let rt = runtime::Builder::new_current_thread() //Builder for a current-thread
                                                                     //runtime.
        .enable_all() //Enable all features.
        .build() //Build the runtime instance.
        .unwrap(); //Unwrap the result.
    rt.block_on(hello("world")); //Block the current thread on the given future.
    
    let mtrt = runtime::Builder::new_multi_thread() //Builder for a multi-thread
                                                                     //runtime.
        .worker_threads(4) //Set the number of worker threads.
        .enable_all() //Enable all features.
        .build() //Build the runtime instance.
        .unwrap(); //Unwrap the result.
    mtrt.block_on(hello("multi world")); //Block the current thread on the given future.
}
*/
