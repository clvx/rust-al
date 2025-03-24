// response is a trait that is implemented for various types to make them usable as responses
// like Html, Json, etc.
// routing is a module that contains the routing primitives like get, post, put, etc.
// Router is a struct that is used to define the routes of the application.
use axum::{response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler));

    // It's called bind because it uses the bind() system call to bind to a socket.
    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap(); // tokio is async

    print!("Server running on port 3000");
    axum::serve(listener, app).await.unwrap(); //axum is async
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
