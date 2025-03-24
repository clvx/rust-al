// response is a trait that is implemented for various types to make them usable as responses
// like Html, Json, etc.
// routing is a module that contains the routing primitives like get, post, put, etc.
// Router is a struct that is used to define the routes of the application.
use axum::{
    extract::{Path, Query, State}, http::HeaderMap, response::Html, routing::get, Router
};
use std::{collections::HashMap, sync::{atomic::AtomicUsize, Arc}};

struct MyCounter {
    counter: AtomicUsize, //interior mutability pattern to share the counter 
                          //between multiple threads
}

#[tokio::main]
async fn main() {

    let shared_counter = Arc::new(MyConfig {
        counter: AtomicUsize::new(0),
    });

    let app = Router::new()
        .route("/", get(handler))
        .route("/book/:id", get(path_extract))
        .route("/book", get(query_extract))
        .route("/headers", get(header_extract))
        .with_state(shared_counter);

    // It's called bind because it uses the bind() system call to bind to a socket.
    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap(); // tokio is async

    print!("Server running on port 3000");
    axum::serve(listener, app).await.unwrap(); //axum is async
}

async fn handler(
    State(config): State<Arc<MyCounter>>
    ) -> Html<String> {
    config.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Html(format!("<h1>Visitor number: {}</h1>", config.counter.load(std::sync::atomic::Ordering::Relaxed)))
}

async fn path_extract(Path(id): Path<u32>) -> Html<String> {
    Html(format!("The book id is {}", id))
}

async fn query_extract(
    Query(params): Query<HashMap<String, String>>
) -> Html<String> {
    Html(format!("{params:#?}"))
}

async fn header_extract(
    headers: HeaderMap
    ) -> Html<String> {
    Html(format!("{headers:#?}"))
}
