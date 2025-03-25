// response is a trait that is implemented for various types to make them usable as responses
// like Html, Json, etc.
// routing is a module that contains the routing primitives like get, post, put, etc.
// Router is a struct that is used to define the routes of the application.
use axum::{
    extract::{Path, Query, State}, http::HeaderMap, response::{Html, IntoResponse}, routing::get, Extension, Json, Router
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;
use std::{collections::HashMap, sync::{atomic::AtomicUsize, Arc}};

struct Counter {
    counter: AtomicUsize, //interior mutability pattern to share the counter 
}

struct Config {
    env: String,
}

#[tokio::main]
async fn main() {

    let shared_counter = Arc::new(Counter {
        counter: AtomicUsize::new(0),
    });

    let shared_config = Arc::new(Config {
        env: "This is configuration".to_string(),
    });

    let app = Router::new()
        .route("/", get(handler))
        .route("/inc", get(increment))
        .nest("/extract", extractors())             // nesting extractors
        .nest("/nest", nesting())                   // nesting services under /svc/1 /svc/2
        .route("/time", get(error_handling))
        .layer(Extension(shared_config))            // shared configuration
        .layer(Extension(shared_counter))          // shared counter
        .fallback_service(ServeDir::new("web"));      // serve static files from the web directory

    // It's called bind because it uses the bind() system call to bind to a socket.
    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap(); // tokio is async

    print!("Server running on port 3000");
    axum::serve(listener, app).await.unwrap(); //axum is async
}

//-------------------------------------------------------------------------
// Index handler ----------------------------------------------------------
// Handler functions are async functions that return a response.
// The response can be a string, a file, a json, etc.
// The response is wrapped in a response type like Html, Json, etc.
// The handler functions can take arguments like extractors, state, etc.
// The handler functions can also have a return type like Result, Option, etc.
// The handler functions can also have a state that is shared between all the requests.
// The state is passed to the handler function as an argument.
// The state is shared between all the requests and can be used to store shared data.
// The state is passed to the handler function as an argument.

async fn handler() -> Html<String> {
    println!("Sending GET request to /inc");
    let current_count = reqwest::get("http://localhost:3000/inc")
        .await
        .unwrap()
        .json::<IncrementResponse>() // deserializing the response
        .await
        .unwrap();
    Html(format!("<h3>/inc counter: {} {}</h3>", current_count.counter, current_count.env))
}

#[derive(Serialize, Deserialize)]
struct IncrementResponse {
    env: String,
    counter: usize,
}

async fn increment(
    Extension(counter): Extension<Arc<Counter>>,
    Extension(config): Extension<Arc<Config>>
    ) -> Json<IncrementResponse> {
    let response = IncrementResponse {
        env: config.env.clone(),
        counter: counter.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
    };
    Json(response)
}
//-------------------------------------------------------------------------
// Extractors -------------------------------------------------------------
// Extractors are used to extract data from the request like path, query, headers, etc.
// Extractors are used as arguments to the handler functions.
// Extractors are async functions that return the extracted data.

fn extractors() -> Router {
    Router::new()
        .route("/book/:id", get(path_extract))      // extractor for path
        .route("/book", get(query_extract))         // extractor for query
        .route("/headers", get(header_extract))     // extractor for headers
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

//-------------------------------------------------------------------------
// Nesting routers allows you to define the routes of the application in a modular way.
// Each service can have its own router and the main router can nest these routers.
// This makes the code more modular and easier to manage.
// The subrouter can have its own state and configuration.

struct SubRouterState(String); 

fn nesting() -> Router {
    Router::new()
        .nest("/1", service_one())  // nesting subrouter 1
        .nest("/2", service_two())  // nesting subrouter 2
}

fn service_one() -> Router {
    let state = Arc::new(SubRouterState("Subrouter state".to_string())); // state for the subrouter
    Router::new().route("/", get(sv1_handler)
        .with_state(state)
        )
}

async fn sv1_handler(
    Extension(counter): Extension<Arc<Counter>>,
    State(state): State<Arc<SubRouterState>>,
    ) -> Html<String> {
    counter.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Html(format!("<h1>Service One <br>
            Visitor number: {}<br>
            {}</h1>", 
            counter.counter.load(std::sync::atomic::Ordering::Relaxed),
            state.0)
        )
}

fn service_two() -> Router {
    Router::new().route("/", get(||async {Html("Service Two".to_string())}))
}

//-------------------------------------------------------------------------
// Error handling ---------------------------------------------------------

// Error handling is done by returning a Result from the handler function.
// IntoResponse is a trait that is implemented for various types to make them usable as responses
// like Html, Json, etc.
// The error is returned as a tuple of (StatusCode, String).
async fn error_handling() -> Result<impl IntoResponse, (StatusCode, String)> {
    let start = std::time::SystemTime::now();
    let seconds_wrapped = start
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Bad clock".to_string()))?
        .as_secs() % 3;
    let divided = 100u64.checked_div(seconds_wrapped)
        .ok_or((StatusCode::BAD_REQUEST, "div by 0".to_string()))?;

    Ok(Json(divided))
}
