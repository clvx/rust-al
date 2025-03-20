use crate::errors::CollectorError;
use shared_data::{decode_response_v1, CollectorResponseV1, DATA_COLLECTOR_ADDRESS};
use std::collections::VecDeque;
use std::io::{Read, Write};

/*
pub fn send_command(bytes: &[u8]) -> Result<(), CollectorError> {
    let mut stream = std::net::TcpStream::connect(DATA_COLLECTOR_ADDRESS)
        .map_err(|_| CollectorError::UnableToConnect)?;
    stream.write_all(&bytes)
        .map_err(|_| CollectorError::UnableToSend)?;

    Ok(())
}
*/

//send_queue sends all the commands in the queue to the data collector in a single connection.
pub fn send_queue(queue: &mut VecDeque<Vec<u8>>) -> Result<(), CollectorError>{
    // connect
    let mut stream = std::net::TcpStream::connect(DATA_COLLECTOR_ADDRESS)
        .map_err(|_| CollectorError::UnableToConnect)?;

        // Send every queue item
        let mut buf = vec![0u8; 512]; // Buffer for reading
        while let Some(command) = queue.pop_front() { // Get the next command
        // Send the command and if it fails, put it back in the queue and return an error
        if stream.write_all(&command).is_err() {
            queue.push_front(command);
            return Err(CollectorError::UnableToSend);
        }
        // Read the response
        let bytes_read = stream.read(&mut buf).map_err(|_| CollectorError::UnableToReceive)?;
        // if no bytes are read, put the command back in the queue and return an error
        if bytes_read == 0 {
            queue.push_front(command);
            return Err(CollectorError::UnableToReceive);
        }
        // Decode the response
        let ack = decode_response_v1(&buf[0..bytes_read]);
        // if the response is not an ack, put the command back in the queue and return an error
        if ack != CollectorResponseV1::Ack(0) {
            queue.push_front(command);
            return Err(CollectorError::UnableToReceive);
        } else {
            // Ack received
            println!("Ack received");
        }
    }
    Ok(())
}
