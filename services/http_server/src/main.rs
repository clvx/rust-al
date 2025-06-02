// response is a trait that is implemented for various types to make them usable as responses
// like Html, Json, etc.
// routing is a module that contains the routing primitives like get, post, put, etc.
// Router is a struct that is used to define the routes of the application.
use axum::{
    extract::{Path, Query, Request, State}, http::HeaderMap, middleware::{self, Next}, response::{Html, IntoResponse}, routing::get, Extension, Json, Router
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use tower::{ServiceBuilder, limit::ConcurrencyLimitLayer};
use tower_http::{compression::CompressionLayer, cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use tracing::{debug, error, info};
use std::{collections::HashMap, sync::{atomic::AtomicUsize, Arc}};

struct Counter {
    counter: AtomicUsize, //interior mutability pattern to share the counter 
}

struct Config {
    env: String,
}



#[tokio::main]
async fn main() {

    // Setup default tracing
    tracing_subscriber::fmt::init();

    info!("Starting the server...");

    let shared_counter = Arc::new(Counter {
        counter: AtomicUsize::new(0),
    });

    let shared_config = Arc::new(Config {
        env: "This is configuration".to_string(),
    });

    // service creates a middleware stack using `ServiceBuilder` from the 
    // `tower` library. Middleware layers are applied to incoming requests 
    // before they reach the application logic and can modify requests, 
    // responses, or enforce policies.
    //
    // ServiceBuilder is a builder pattern object that allows you to chain 
    // .layer() calls to add middleware layers.
    // Once all the desired layers are added, calling `.into_inner()` finalizes 
    // the builder and produces the actual `Service` object.
    let service = ServiceBuilder::new()
        .layer(Extension(shared_config))  // Adds the shared_config as an 
                                          // extension making it available to 
                                          // all request handlers via the 
                                          // Extension extractor.
        .layer(Extension(shared_counter))
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http()) // TraceLayer is used for logging requests and
                                           // responses, including their
                                           // timing and status codes.
                                           // Needs RUST_LOG=debug 
        .layer(CorsLayer::permissive())
        .layer(ConcurrencyLimitLayer::new(100));
                                            // responses

    let app = Router::new()
        .route("/", get(handler))
        .merge(increment())
        .merge(extractors())                        // merging extractors
        .nest("/nest", nesting())                   // nesting services under /svc/1 /svc/2
        .route("/time", get(error_handling))
        .layer(service.into_inner())  // used to consume the ServiceBuilder 
                                      // and retrieve the constructed 
                                      // middleware stack as a Service instance 
        .fallback_service(ServeDir::new("web"));     // serve static files from the web directory

    // It's called bind because it uses the bind() system call to bind to a socket.
    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .unwrap(); // tokio is async

    info!("Server running on port 3000");
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

    let response = reqwest::Client::new()
        .get("http://localhost:3000/inc")   // making a GET request to /inc
        .header("x-request-id", "123")      // adding auth header
        .send()                             // calling send to send the request
        .await                              // waiting for the response
        .unwrap()                           // unwrapping the response
        .json::<IncrementResponse>()        // deserializing the response
        .await                              // waiting for the deserialization   
        .unwrap();                          // unwrapping the deserialization
    Html(format!("<h3>/inc counter: {} {}, x-request-id: {}</h3>", response.counter, response.env, response.auth))
}

#[derive(Serialize, Deserialize)]
struct IncrementResponse {
    env: String,
    counter: usize,
    auth: String,
}

fn increment() -> Router {
    Router::new()
        .route("/inc", get(inc))
        .route_layer(middleware::from_fn(auth))    // middleware for authentication
}

async fn inc(
    Extension(counter): Extension<Arc<Counter>>,
    Extension(config): Extension<Arc<Config>>,
    Extension(auth): Extension<AuthHeader>
    ) -> Json<IncrementResponse> {
    let response = IncrementResponse {
        env: config.env.clone(),
        counter: counter.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
        auth: auth.id.clone(),
    };
    Json(response)
}
//-------------------------------------------------------------------------
// Extractors -------------------------------------------------------------
// Extractors are used to extract data from the request like path, query, headers, etc.
// Extractors are used as arguments to the handler functions.
// Extractors are async functions that return the extracted data.

fn extractors() -> Router {
    // debug! only works if the log level is set to debug or lower
    // You can set the log level by setting the RUST_LOG environment variable
    debug!("Setting up extractors...");
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

//-------------------------------------------------------------------------
// Auth middleware --------------------------------------------------------
// Middleware is a function that takes a request and returns a response.
// Middleware can be used to modify the request or response.
// Middleware can be used to add authentication, logging, etc.
// Middleware can be used to add shared state to the request.
// Middleware can be used to add shared configuration to the request.
// Middleware can be used to add shared data to the request.

#[derive(Clone)]
struct AuthHeader { id: String } // AuthHeader needs to be cloneable because it 
                                 // is stored in the request extensions

async fn auth(
    headers: HeaderMap,
    mut req: Request,       // req needs to be mutable because we are adding the AuthHeader to it
    next: Next,             // Next is a type alias for a function that takes a request and returns a response.
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // TODO: Fix this to not succeed when there isn't a header
    if let Some(header) =  headers.get("x-request-id") {
        // Validate the header
        let header = header.to_str().unwrap();
        if header == "123" {
            req.extensions_mut().insert(AuthHeader { id: header.to_string() });
            return Ok(next.run(req).await);
        }
    }
    error!("Unauthorized request: missing or invalid x-request-id header");
    Err((StatusCode::UNAUTHORIZED, "Unauthorized".to_string()))
}
