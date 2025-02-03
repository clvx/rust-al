use sqlx::{Pool, Sqlite};
use tokio::net::{TcpListener, TcpStream};
use shared_data::{decode_v1, encode_response_v1, CollectorCommandV1, CollectorResponseV1, DATA_COLLECTOR_ADDRESS};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::net::SocketAddr;



pub async fn data_collector(cnn: Pool<Sqlite>) -> anyhow::Result<()> {
    // Listen for TCP connections on the data collector address
    let listener = TcpListener::bind(DATA_COLLECTOR_ADDRESS).await?;

    // Loop forever, accepting connections
    loop {
        // Wait for a new connection
        let cnn = cnn.clone();
        let (socket, address) = listener.accept().await?;
        tokio::spawn(new_connection(socket, address, cnn));
    }
}

async fn new_connection(mut socket: TcpStream, address: SocketAddr, cnn: Pool<Sqlite>) {
    println!("New connection from {address:?}");
    let mut buf = vec![0u8; 1024];
    loop {
        let n = socket
            .read(&mut buf)
            .await
            .expect("failed to read data from socket");

        if n == 0 {
            println!("No data received - connection closed");
            return;
        }

        println!("Received {n} bytes");
        let received_data = decode_v1(&buf[0..n]);
        match received_data {
            (timestampt, CollectorCommandV1::SubmitData { collector_id, total_memory, used_memory, average_cpu_usage }) => {
                let collector_id = uuid::Uuid::from_u128(collector_id);
                let collector_id = collector_id.to_string();

                // Insert the data into the database
                let result = sqlx::query("INSERT INTO timeseries (collector_id, received, total_memory, used_memory, average_cpu) VALUES (?, ?, ?, ?, ?)")
                    .bind(&collector_id)
                    .bind(timestampt)
                    .bind(total_memory as i64)
                    .bind(used_memory as i64)
                    .bind(average_cpu_usage)
                    .execute(&cnn)
                    .await;

                if result.is_err() {
                    eprintln!("Failed to insert data: {result:?}");
                } else { // Send an ACK
                    let ack = CollectorResponseV1::Ack(0);
                    let bytes = encode_response_v1(ack);
                    socket.write_all(&bytes).await.unwrap();
                }
            }
        }
    }
}
