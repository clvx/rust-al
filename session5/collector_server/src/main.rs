mod collector;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Read the .env file and obtain the database URL
    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;

    // Get a database connection pool
    let pool = sqlx::SqlitePool::connect(&db_url).await?;

    let handle = tokio::spawn(collector::data_collector(pool));

    // Wait for the data collector to finish
    handle.await??; // Two question marks - we're unwrapping the task result, and the result from running the collector.
    Ok(())
}

