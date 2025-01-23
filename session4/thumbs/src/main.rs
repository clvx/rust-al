use std::net::SocketAddr;

use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use tokio::task::spawn_blocking;
use sqlx::{prelude::FromRow, Pool, Row, Sqlite};
use tokio_util::io::ReaderStream;

use axum::{
    body::StreamBody, extract::{Multipart, Path}, http::{header, HeaderMap}, response::{Html, IntoResponse}, routing::{get, post}, Extension, Form, Json, Router
};



#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // Read the .env file and build environment variables
    dotenv::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = sqlx::SqlitePool::connect(&db_url).await?;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    // Catch up on missing thumbnails
    fill_missing_thumbnail(&pool).await?;

    // Build Axum with an "extension" to hold the database connection pool
    let app = Router::new()
        .route("/", get(index_page))
        .route("/upload", post(uploader))
        .route("/image/:id", get(get_image))
        .route("/thumb/:id", get(get_thumbnail))
        .route("/images", get(list_images))
        .route("/search", post(search_images))
        .layer(Extension(pool));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

/*
async fn test(Extension(pool): Extension<sqlx::SqlitePool>) -> String {
    let result = sqlx::query( "SELECT COUNT(id) FROM images")
        .fetch_one(&pool)
        .await
        .unwrap();
    let count: i64 = result.get(0);
    format!("There are {} images in the database", count)
}
*/


async fn index_page() -> Html<String> {
    let path = std::path::Path::new("src/index.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}

async fn insert_image_into_database(pool: &sqlx::SqlitePool, tags: &str) -> anyhow::Result<i64> {
    let row = sqlx::query(
        "INSERT INTO images (tags) VALUES (?) RETURNING id"
    )
    .bind(tags)
    .fetch_one(pool)
    .await?;

    Ok(row.get(0))
}

async fn save_image(id: i64, image_bytes: &[u8]) -> anyhow::Result<()> {
    // Check that the images folder exists and is a directory
    // If it doesn't, create it.
    let base_path = std::path::Path::new("images");
    if !base_path.exists() || !base_path.is_dir() {
        tokio::fs::create_dir_all(base_path).await?;
    }

    // Use "join" to create pa path to the image file. Join is platform aware,
    // it will handle the differences between Windows and Linux.
    let image_path = base_path.join(format!("{}.jpg", id));
    if image_path.exists(){
        // The file existss. That shouldn't happen.
        anyhow::bail!("File already exists");
    }

    //Write the image to the file.
    tokio::fs::write(image_path, image_bytes).await?;
    Ok(())
}

async fn uploader(
    Extension(pool): Extension<sqlx::SqlitePool>,
    mut multipart: Multipart,
    ) -> Html<String> {
    let mut tags = None; // "None" means "no tags yet"
    let mut image = None;
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        match name.as_str() {
            "tags" => tags = Some(String::from_utf8(data.to_vec()).unwrap()), // Using Some means we can check we received it
            "image" => image = Some(data.to_vec()),
            _ => panic!("Unknown field: {name}"),
        }
    }

    if let (Some(tags), Some(image)) = (tags, image) { // Destructuring both Options at once
        let new_image_id = insert_image_into_database(&pool, &tags).await.unwrap();
        save_image(new_image_id, &image).await.unwrap();
        let _ = spawn_blocking(move || {
            make_thumbnail(new_image_id)
        }).await.unwrap();
    } else {
        panic!("Missing field");
    }

    let path = std::path::Path::new("src/redirect.html");
    let content = tokio::fs::read_to_string(path).await.unwrap();
    Html(content)
}

async fn get_image(Path(id): Path<i64>) -> impl IntoResponse {
    let filename = format!("images/{}.jpg", id);
    let attachment = format!("filename={filename}");
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("image/jpeg"),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        header::HeaderValue::from_str(&attachment).unwrap(),
    );
    let file = tokio::fs::File::open( &filename).await.unwrap();
    axum::response::Response::builder()
        .header(header::CONTENT_TYPE, header::HeaderValue::from_static("image/jpeg"))
        .header(header::CONTENT_DISPOSITION, header::HeaderValue::from_str(&attachment).unwrap())
        .body(StreamBody::new(ReaderStream::new(file))) // explanation
                                                                                                           // on
                                                                                                           // min
                                                                                                           // 40:00
        .unwrap()
}

async fn get_thumbnail(Path(id): Path<i64>) -> impl IntoResponse {
    let filename = format!("images/{}_thumb.jpg", id);
    let attachment = format!("filename={filename}");
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("image/jpeg"),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        header::HeaderValue::from_str(&attachment).unwrap(),
    );
    let file = tokio::fs::File::open( &filename).await.unwrap();
    axum::response::Response::builder()
        .header(header::CONTENT_TYPE, header::HeaderValue::from_static("image/jpeg"))
        .header(header::CONTENT_DISPOSITION, header::HeaderValue::from_str(&attachment).unwrap())
        .body(StreamBody::new(ReaderStream::new(file))) // explanation
                                                                                                           // on
                                                                                                           // min
                                                                                                           // 40:00
        .unwrap()
}

fn make_thumbnail(id: i64) -> anyhow::Result<()> {
    let image_path = format!("images/{}.jpg", id);
    let thumbnail_path = format!("images/{}_thumb.jpg", id);
    let image_bytes = std::fs::read(&image_path)?;
    let image = if let Ok(format) = image::guess_format(&image_bytes) {
        image::load_from_memory_with_format(&image_bytes, format)?
    } else {
        image::load_from_memory(&image_bytes)?
    };
    let thumbnail = image.thumbnail(100, 100);
    thumbnail.save(thumbnail_path)?;
    Ok(())
}

async fn fill_missing_thumbnail(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    let mut rows = sqlx::query("SELECT id FROM images")
        .fetch(pool);

    while let Some(row) = rows.try_next().await?{
        let id = row.get::<i64, _>(0);
        let thumbnail_path = format!("images/{}_thumb.jpg", id);
        if !std::path::Path::new(&thumbnail_path).exists() {
            let _ = spawn_blocking(move || {
                make_thumbnail(id)
            }).await?;
        }
    }
    Ok(())
}

#[derive(Deserialize, Serialize, FromRow, Debug)]
struct ImageRecord {
    id: i64,
    tags: String,
}

async fn list_images(Extension(pool): Extension<sqlx::SqlitePool>) -> Json<Vec<ImageRecord>> {
    sqlx::query_as::<_, ImageRecord>("SELECT id, tags FROM images ORDER BY id")
        .fetch_all(&pool)
        .await
        .unwrap()
        .into()
}


#[derive(Deserialize)]
struct Search {
    tags: String
}

async fn search_images(Extension(pool): Extension<sqlx::SqlitePool>, Form(form): Form<Search>) -> Html<String> {
    let tag = format!("%{}%", form.tags);

    let rows = sqlx::query_as::<_, ImageRecord>("SELECT id, tags FROM images WHERE tags LIKE ? ORDER BY id")
        .bind(tag)
        .fetch_all(&pool)
        .await
        .unwrap();

    let mut results = String::new();
    for row in rows {
        results.push_str(&format!("<a href=\"/image/{}\"><img src='/thumb/{}' /></a><br />", row.id, row.id));
    }

    let path = std::path::Path::new("src/search.html");
    let mut content = tokio::fs::read_to_string(path).await.unwrap();
    content = content.replace("{results}", &results);

    Html(content)
}
