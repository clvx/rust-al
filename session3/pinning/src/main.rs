use async_recursion::*;
use std::{pin::Pin, future::Future};

// The function is marked with the async_recursion macro so that it can call itself recursively
// when it is an async function. The macro is used to avoid the error "error[E0721]: recursion in
// an `async fn` requires boxing".
//
// pinning the future returned by the recursive call to the stack is not possible because The
// future returned by the recursive call is not 'static. The async_recursion macro solves this
// problem by boxing the future returned by the recursive call.
#[async_recursion]
async fn fibonacci(n:u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n-1).await + fibonacci(n-2).await
    }
}

async fn one() {
    println!("one");
}

async fn two() {
    println!("two");
}

async fn call_one_of_them(n:u32) -> Pin<Box<dyn Future<Output = ()>>> {
    match n {
        1 => Box::pin(one()),
        2 => Box::pin(two()),
        _ => panic!("n must be 1 or 2")
    }
}

#[tokio::main]
async fn main() {
    println!("fibonacci(10) = {}", fibonacci(10).await);

    let future = async {
        println!("Hello World");
    };
    tokio::pin!(future);
    (&mut future).await;

}
