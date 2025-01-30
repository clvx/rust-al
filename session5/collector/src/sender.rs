use crate::errors::CollectorError;
use shared_data::{CollectorCommandV1, DATA_COLLECTOR_ADDRESS};
use std::collections::VecDeque;
use std::io::Write;

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

    // send every queue item
    while let Some(command) = queue.pop_front() {
        if stream.write_all(&command).is_err() {
            queue.push_front(command);
            return Err(CollectorError::UnableToSend);
        }
    }
    Ok(())
}
