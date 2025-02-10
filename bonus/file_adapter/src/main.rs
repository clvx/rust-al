use axum::http::{header, HeaderMap, StatusCode};
use axum::{body::StreamBody, response::IntoResponse, routing::get, Router};
use std::net::SocketAddr;
use tokio::io::BufReader;
use pin_project_lite::pin_project;

pin_project! {
    struct ToUpper {
        #[pin]
        stream: tokio_stream::wrappers::LinesStream<BufReader<tokio::fs::File>>,
    }
}

impl ToUpper {
    fn new(stream: tokio_stream::wrappers::LinesStream<BufReader<tokio::fs::File>>) -> Self {
        Self { stream }
    }
}

impl tokio_stream::Stream for ToUpper {
    type Item = std::io::Result<String>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.project()
            .stream
            .poll_next(cx)
            .map(|opt| opt.map(|res| res.map(|line| line.to_uppercase() + "\n")))
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> impl IntoResponse {
    use tokio::io::AsyncBufReadExt;

    // `File` implements `AsyncRead`
    let file = match tokio::fs::File::open("Cargo.toml").await {
        Ok(file) => file,
        Err(e) => return Err((StatusCode::NOT_FOUND, format!("file not found: {}", e))),
    };
    // convert `AsyncRead` into a buffered reader, then a line stream, then your adapter
    let stream = BufReader::new(file).lines();
    let stream = tokio_stream::wrappers::LinesStream::new(stream);
    let stream = ToUpper::new(stream);

    // convert the `Stream` into an `axum::body::HttpBody`
    let body = StreamBody::new(stream);

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("text/toml; charset=utf-8"),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        header::HeaderValue::from_str("attachment; filename=\"Cargo.toml\"").unwrap(),
    );

    Ok((headers, body))
}
