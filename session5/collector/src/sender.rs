use crate::errors::CollectorError;
use shared_data::{CollectorCommandV1, DATA_COLLECTOR_ADDRESS};
use std::io::Write;


/*
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CollectorError {
    #[error("Failed to connect to server")]
    UnableToConnect,
    #[error("Failed to send data")]
    UnableToSend,
}
*/

pub fn send_command(command: CollectorCommandV1) -> Result<(), CollectorError> {
    let bytes = shared_data::encode_v1(command);
    println!("Encoded {} bytes", bytes.len());
    let mut stream = std::net::TcpStream::connect(DATA_COLLECTOR_ADDRESS)
        .map_err(|_| CollectorError::UnableToConnect)?;
    stream.write_all(&bytes)
        .map_err(|_| CollectorError::UnableToSend)?;

    Ok(())
}
