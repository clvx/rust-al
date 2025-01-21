use axum::{routing::{get,post}, Router};
use tokio::net::TcpListener;
use axum::response::Html;
use serde::Serialize;

//async fn say_hello_text() -> &'static str {
async fn hello() -> Html<&'static str> {
    //"Hello, World!"
    const HTML: &str = include_str!("hello.html"); // include_str! is a macro that reads a file at
                                                   // compile time and includes it as a string
                                                   // literal.
    //Html("<h1>Hello, World!</h1>")
    Html(HTML)
}

async fn hello_from_file() -> Html<String> {
    let path = std::path::Path::new("src/hello.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}

#[derive(Serialize)]
struct HelloJson {
    message: String,
}

async fn hello_json() -> axum::Json<HelloJson> {
    let message = HelloJson{message: "Hi from JSON".to_string() };
    axum::Json(message)
}

async fn hello_post() -> Html<String> {
    Html("<h1>Hello, POST!</h1>".to_string())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let router = Router::new()
        .route("/", get(hello)) // .route("/", get(say_hello_text) is
                                                               // a shorthand for .route("/", get(say_hello_text.into_handler()))
                                                               // .into_handler() is a method that
                                                               // converts a function into a
                                                               // handler.
                                                               // get() is a function that creates
                                                               // axum::routing::Route that matches
                                                               // GET requests.
        .route("/file", get(hello_from_file))
        .route("/json", get(hello_json))
        .route("/post", post(hello_post));
    let listener = TcpListener::bind("127.0.0.1:3000").await?; // bind() is a method that binds a
                                                               // listener to an address.
                                                               // TcpListener::bind() is a function
                                                               // that binds a listener to an address
                                                               // and returns a future that resolves
                                                               // to a TcpListener.
                                                               // await? is a macro that unwraps an
                                                               // Option or a Result and returns the
                                                               // inner value or propagates the error.
                                                               // It is a shorthand for .await.unwrap()
                                                               // or .await.expect().
    axum::serve(listener, router).await?; // serve() is a function that serves a router on a
                                          // listener.
                                          // axum::serve() is a function that serves a router
                                          // on a listener and returns a future that resolves
                                          // to a Result.
                                          // await? is a macro that unwraps an Option or a
                                          // Result and returns the inner value or propagates
                                          // the error. It is a shorthand for .await.unwrap()
                                          // or .await.expect().

    Ok(())
}
