// response is a trait that is implemented for various types to make them usable as responses
// like Html, Json, etc.
// routing is a module that contains the routing primitives like get, post, put, etc.
// Router is a struct that is used to define the routes of the application.
use axum::{
    extract::{Path, Query}, http::HeaderMap, response::Html, routing::get, Extension, Router
};
use std::{collections::HashMap, sync::{atomic::AtomicUsize, Arc}};

struct MyCounter {
    counter: AtomicUsize, //interior mutability pattern to share the counter 
                          //between multiple threads
}

struct MyConfig {
    text: String,
}

fn service_one() -> Router {
    Router::new().route("/", get(||async {Html("Service One".to_string())}))
}

fn service_two() -> Router {
    Router::new().route("/", get(||async {Html("Service Two".to_string())}))
}


#[tokio::main]
async fn main() {

    let shared_counter = Arc::new(MyCounter {
        counter: AtomicUsize::new(0),
    });

    let shared_text = Arc::new(MyConfig {
        text: "This is configuration".to_string(),
    });

    let app = Router::new()
        .nest("/1", service_one()) //add router for service one
        .nest("/2", service_one()) //add router for service two
        .route("/", get(handler))
        .route("/book/:id", get(path_extract))
        .route("/book", get(query_extract))
        .route("/headers", get(header_extract))
        .layer(Extension(shared_text))
        .layer(Extension(shared_counter));

    // It's called bind because it uses the bind() system call to bind to a socket.
    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap(); // tokio is async

    print!("Server running on port 3000");
    axum::serve(listener, app).await.unwrap(); //axum is async
}

async fn handler(
    Extension(counter): Extension<Arc<MyCounter>>,
    Extension(config): Extension<Arc<MyConfig>>,
    ) -> Html<String> {
    counter.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Html(format!("<h1>{} - Visitor number: {}</h1>", 
            config.text,
            counter.counter.load(std::sync::atomic::Ordering::Relaxed))
        )
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
