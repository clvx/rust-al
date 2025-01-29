use std::{fs::File, io::Write};

#[derive(Debug)]
struct OurData {
    number: u16,
    tag: String,
}

/*
 * main function implements a protocol spec for writing and reading a struct to a file.
 *
 *  Offset	Size (Bytes)	Field	        Description
 *  ----------------------------------------------------
 *  0	    2	            number (u16)	A 16-bit unsigned integer stored in little-endian format.
 *  2	    8	            length (u64)	A 64-bit unsigned integer representing the length of the string (in bytes), also in little-endian format.
 *  10	    length	        tag (String)	The actual UTF-8 encoded string data, stored as raw bytes.
 *  ----------------------------------------------------
 *
 *  binary file: bytes.bin
 *  ----------------------------------------------------
 *  Offset	Bytes(Hex)                  	    Decoded Value
 *  ----------------------------------------------------
 *  0-1	    0C 00	                            12 (u16)
 *  2-9	    0B 00 00 00 00 00 00 00	11          (u64, length of "Hello World")
 *  10-20	48 65 6C 6C 6F 20 57 6F 72 6C 64	"Hello World" (UTF-8 bytes)
*/
fn main() {
    let a = OurData {
        number: 12,
        tag: "Hello World".to_string(),
    };

    // Write the record in parts
    let mut file = File::create("bytes.bin").unwrap();

    // Write the number and check that 2 bytes were written
    assert_eq!(file.write(&a.number.to_le_bytes()).unwrap(), 2);

    // Write the string length IN BYTES and check that 8 bytes were written
    let len = a.tag.as_bytes().len();
    assert_eq!(file.write(&(len as u64).to_le_bytes()).unwrap(), 8);

    // Write the string and check that the correct number of bytes were written
    assert_eq!(file.write(a.tag.as_bytes()).unwrap(), len);

    ///// READ THE DATA BACK
    // Read the whole file as bytes.
    let bytes = std::fs::read("bytes.bin").unwrap();

    // Read the number
    let number = u16::from_le_bytes(bytes[0..2].try_into().unwrap());

    // Read the string length
    let length = u64::from_le_bytes(bytes[2..10].try_into().unwrap());

    // Decode the string
    let tag = std::str::from_utf8(&bytes[10..(10 + length as usize)]).unwrap();

    let a = OurData {
        number,
        tag: tag.to_string(),
    };
    println!("{a:?}");
}
