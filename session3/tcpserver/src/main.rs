use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    spawn,
};


// tcp_client function that connects to the server and sends a message to the server and
// receives a response
async fn tcp_client() -> anyhow::Result<()> { 
    let mut stream = tokio::net::TcpStream::connect("::1:1337").await?; // connect to the
                                                                                         // server
    println!("Connected to the server!");
    stream.write_all(b"Hello, world!").await?; // send a message to the server
    let mut buf = vec![0; 1024]; // allocate a buffer to store the data read from the
                                         // socket
    let bytes_read = stream.read(&mut buf).await?; // read data from the socket into the
                                                          // buffer and store the number of
                                                          // bytes read
    println!("Received: {:?}", String::from_utf8_lossy(&buf[..bytes_read])); // print the
                                                                             // message received from the
                                                                             // server to
                                                                             // the console 

    Ok(())
}

// client_runner function that runs the tcp_client function every second
async fn client_runner() {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        let _ = tcp_client().await;
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    tokio::spawn(client_runner()); // spawn the tcp client runner task

    let listener = TcpListener::bind(":::1337").await?; // bind the listener to the
                                                                     // address

    loop {
        let (mut socket, address) = listener.accept().await?; // accept
                                                                                     // incoming
                                                                                     // connections
        spawn(async move { // spawn a new task to handle the connection concurrently with
                                   // the main loop of the server application
            println!("Connection from {address:?}");
            let mut buf = vec![0; 1024]; // allocate a buffer to store the data read from the
                                                 // socket
            loop { // read data from the socket and write it back
                let n = socket.
                    read(&mut buf)
                    .await
                    .expect("Failed to read data from socket"); // read data from the socket
                                                                     // into the buffer and store 
                                                                     // the number of bytes read
                if n == 0 {
                    return;
                } // if no data was read, the connection was closed

                socket.write_all(&buf[0..n])
                    .await
                    .expect("Failed to write data to socket"); // write the data read from the
                                                                     // socket back to the socket
            }
        });
    }

    //Ok(())

}
