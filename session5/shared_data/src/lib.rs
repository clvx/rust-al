use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};


/*
 * Bytes 	    Name 	                Description
 * --------------------------------------------------
 * 0-1 	        Magic Number 	        Sending a magic number is a common way to 
 *                                      ensure that the data you're receiving is what you expect.
 * 2-3 	        Version Number 	        We'll start with version 1. We're going 
 *                                      to use two bytes, so we have lots of room 
 *                                      for future versions. If we somehow use 65,535 
 *                                      versions, we'll mark a version as indicating 
 *                                      that the next bytes are a sub-version!
 * 4-7 	        Timestamp 	            We'll use a 32-bit unsigned integer to represent 
 *                                      the number of seconds since the Unix epoch. 
 *                                      This will give us a range of 1970-01-01 to 2106-02-07.
 * 8-11 	    Payload size 	        We'll use a 32-bit unsigned integer to represent 
 *                                      the size of the payload.
 * 12+ 	        Payload 	            We'll start with JSON and move to something 
 *                                      more efficient.
 * End-4 - End 	CRC32 	                We'll use a CRC32 checksum to ensure that 
 *                                      the data we received is the data we expected. 
 *                                      We'll use the crc32fast crate to provide this functionality.
*/

pub const DATA_COLLECTOR_ADDRESS: &str = "127.0.0.1:9004";
const MAGIC_NUMBER: u16 = 1234;
const VERSION_NUMBER: u16 = 1;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CollectorCommandV1 {
    SubmitData {
        collector_id: u128, // To be converted from a UUID
        total_memory: u64,
        used_memory: u64,
        average_cpu_usage: f32,
    },
}

//unix_now gets the current time in seconds since the Unix epoch.
fn unix_now() -> u32 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs() as u32
}

//encode_v1 encodes a CollectorCommandV1 into a Vec<u8> following the protocol spec.
pub fn encode_v1(command: &CollectorCommandV1) -> Vec<u8> {
    //let json = serde_json::to_string(&command).unwrap();
    //let json_bytes = json.as_bytes();
    let payload_bytes = bincode::serialize(command).unwrap();
    let crc = crc32fast::hash(&payload_bytes);
    let payload_size = payload_bytes.len() as u32;
    let timestamp = unix_now();

    // Encode into bytes
    let mut result = Vec::with_capacity(140);
    result.extend_from_slice(&MAGIC_NUMBER.to_be_bytes());
    result.extend_from_slice(&VERSION_NUMBER.to_be_bytes());
    result.extend_from_slice(&timestamp.to_be_bytes());
    result.extend_from_slice(&payload_size.to_be_bytes());
    result.extend_from_slice(&payload_bytes);
    result.extend_from_slice(&crc.to_be_bytes());
    result
}

//decode_v1 decodes a Vec<u8> into a CollectorCommandV1 following the protocol spec.
pub fn decode_v1(bytes: &[u8]) -> (u32, CollectorCommandV1) {
    let magic_number = u16::from_be_bytes([bytes[0], bytes[1]]);
    let version_number = u16::from_be_bytes([bytes[2], bytes[3]]);
    let timestamp = u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    let payload_size = u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
    let payload = &bytes[12..12 + payload_size as usize]; // 12 is the start of the payload
    //crc is the last 4 bytes. We can get it by taking the last 4 bytes of the slice.
    let crc = u32::from_be_bytes([
        bytes[12 + payload_size as usize],
        bytes[13 + payload_size as usize],
        bytes[14 + payload_size as usize],
        bytes[15 + payload_size as usize],
    ]);

    // Verify the magic number
    assert_eq!(magic_number, MAGIC_NUMBER);

    // Verify the version number
    assert_eq!(version_number, VERSION_NUMBER);

    // Verify the CRC
    let computed_crc = crc32fast::hash(payload);
    assert_eq!(crc, computed_crc);

    // Decode the payload
    (timestamp, bincode::deserialize(payload).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode() {
        let command = CollectorCommandV1::SubmitData {
            collector_id: 123,
            total_memory: 100,
            used_memory: 50,
            average_cpu_usage: 0.5,
        };
        let encoded = encode_v1(&command);
        let (timestamp, decoded) = decode_v1(&encoded);
        assert_eq!(decoded, command);
        assert!(timestamp > 0);
    }
}
