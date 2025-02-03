use thiserror::Error;

#[derive(Error, Debug)]
pub enum CollectorError {
    #[error("Failed to connect to server")]
    UnableToConnect,
    #[error("Failed to send data")]
    UnableToSend,
    #[error("Failed to receive data")]
    UnableToReceive,
}
