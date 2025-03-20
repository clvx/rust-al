//use sqlx::Row;
use sqlx::FromRow;
use futures::stream::TryStreamExt;

#[derive(Debug, FromRow)]
struct Message {
    id: i64,
    message: String,
}

async fn update_message(id: i64, message: &str, pool: &sqlx::SqlitePool) -> anyhow::Result<()> {
    sqlx::query("UPDATE messages SET message = ? WHERE id = ?")
        .bind(message)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?; // Load .env file to environment
    let db_url = std::env::var("DATABASE_URL")?; // Get DATABASE_URL from environment

    let pool = sqlx::SqlitePool::connect(&db_url).await?; // pool is a connection
                                                                        // pool to the database
    // Run Migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    /*
    //long way
    let messages = sqlx::query("SELECT id, message FROM messages")
        .map(|row: sqlx::sqlite::SqliteRow| {
            let id: i64 = row.get(0);
            let message: String = row.get(1);
            (id, message)
        })
        .fetch_all(&pool)
        .await?;

    for (id, message) in messages {
        println!("{}: {}", id, message);
    }
    */

    //fetch_all fetches all rows from the query as an array
    let messages = sqlx::query_as::<_, Message>("SELECT id, message FROM messages")
        .fetch_all(&pool) // fetch_all returns a Future that resolves to a Vec<Message>
        .await?;

    println!("{:?}", messages);

    update_message(4, "Hello all", &pool).await?;

    //fetch returns a Stream that yields rows as they are received
    let mut message_stream = sqlx::query_as::<_, Message>("SELECT id, message FROM messages")
        .fetch(&pool); // fetch returns a Stream that yields rows as they are received
    while let Some(message) = message_stream.try_next().await? {
        println!("{:?}", message);
    }
                                                                        

    Ok(())
}
