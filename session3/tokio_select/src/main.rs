use std::time::Duration;

async fn do_work(){
    tokio::time::sleep(Duration::from_secs(2)).await;
}

async fn timeout(seconds: f32){
    tokio::time::sleep(Duration::from_secs_f32(seconds)).await;
}

#[tokio::main]
async fn main() {
    tokio::select! { // select! is a macro that allows you to wait for multiple futures to completed
                     // select! will return the first future that completes
        _ = do_work() => println!("do_work() completed"),
        _ = timeout(0.1) => println!("timeout() completed"),
    }
}
