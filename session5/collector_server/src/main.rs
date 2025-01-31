mod collector;
use tokio::net::TcpListener;
use axum::Extension;
use axum::{Router, routing::get};

mod api;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Read the .env file and obtain the database URL
    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;

    // Get a database connection pool
    let pool = sqlx::SqlitePool::connect(&db_url).await?;

    let handle = tokio::spawn(collector::data_collector(pool.clone()));



    // Start the web server
    let app = Router::new()
        .route("/", get(test))
        .route("/api/all", get(api::show_all))
        .layer(Extension(pool)); // This is the database connection pool
    let addr = TcpListener::bind("localhost:3000").await?;
    axum::serve(addr, app).await?;

    // Wait for the data collector to finish
    handle.await??; // Two question marks - we're unwrapping the task result, and the result from running the collector.
    Ok(())
}

async fn test() -> &'static str {
    "Hello, world!"
}

