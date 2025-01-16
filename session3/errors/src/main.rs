use std::path::Path;
use std::io::Error;
use thiserror::Error;
use serde::Deserialize;



fn read_file(filename: &str) -> Result<String, Error> {
    let file = Path::new(filename);
    std::fs::read_to_string(file)
}

fn file_to_uppercase(filename: &str)-> Result<String, Error>{
    let contents = read_file(filename)?; //? will return the error if there is
                                                            //one
    Ok(contents.to_uppercase())
}

#[derive(Debug, serde::Deserialize)]
struct File {
    name: String,
    size: u64,
}

type GenericResult<T> = Result<T, Box<dyn std::error::Error>>; // this is a type alias

// list_files loads a list of files from a Directory
// it returns a Result type that can hold either a Vec<File> or an errors
// the error type is Box<dyn std::error::Error> which means it can hold any error types
// this is useful when you have multiple error types that you want to returns
// you can also use anyhow::Result which is a more convenient way to handle errors
fn list_files(path: &str) -> GenericResult<Vec<File>> {
//fn list_files() -> Result<Vec<File>, Error> {
    let dir = Path::new(path);
    if dir.is_dir() {
        let mut files = vec![];
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let metadata = std::fs::metadata(&path)?;
            let file = File {
                name: path.display().to_string(),
                size: metadata.len(),
            };
            files.push(file);
        }
        Ok(files)
    } else {
        Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Directory not found")))
    }
    //files // this will return a Vec<File> but the function signature expects 
            // Result<Vec<File>,Error> as there are two possible return types, 
            // we need to wrap the Vec<User> in Ok and 
            // also change the function signature to return GenericResult<Vec<User>>
}

// load_users_anyhow is the same as load_users but uses anyhow instead of Box<DirectoryNotEmpty>
// anyhow is a crate that provides a Result type that can hold any error type
//// it also provides a macro called bail! that returns an error right away
//// it is a more convenient way to handle errors
fn list_files_anyhow (path: &str) -> anyhow::Result<Vec<File>> {
    let dir = Path::new(path);
    if dir.is_dir() {
        let mut files = vec![];
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let metadata = std::fs::metadata(&path)?;
            let file = File {
                name: path.display().to_string(),
                size: metadata.len(),
            };
            files.push(file);
        }
        Ok(files)
    } else {
        anyhow::bail!("Directory not found");
    }
}


#[derive(Error, Debug)]
enum DataError{
    #[error("File not found")]
    ErrFileNotFound,
    #[error("Corrupted data found")]
    CorruptedData,
}

#[derive(Deserialize, Debug)]
struct Data {
    name: String,
}

// load_users_thiserror is the same as load_users but uses thiserror to define custom
// error types
// thiserror is a crate that provides a macro called #[derive(Error)] that can be used
// to define custom error types
fn load_file_thiserror(filename: &str) -> Result<String, DataError> {
    let contents = read_file(filename)
        .map_err(|_| DataError::ErrFileNotFound)?;
    let data: Data = serde_json::from_str(&contents)
        .map_err(|_| DataError::CorruptedData)?;
    Ok(data.name)
}

fn main() {

    //ignoring errors
    let filename = "foo.txt";
    let _ = file_to_uppercase(filename); // _ is used to ignore the result

    //handling errors
    if let Ok(data) = file_to_uppercase(filename) {
        println!("Content: {}", data);
    } else {
        eprintln!("Error reading file");
    }


    //let content = std::fs::read_to_string(file).unwrap(); //unwrap  panics if there
                                                                        //is an error
    let content = read_file(filename); // this returns a
                                                                              // Result type which
                                                                              // is an enum with
                                                                              // two variants Ok
                                                                              // and Err
    match content {
        Ok(data) => println!("Content: {}", data),
        //Err(e) => eprintln!("Error: {:#?}", e), // this will print to stderr
        Err(e) =>  match e.kind() {
            std::io::ErrorKind::NotFound => println!("File not found"),
            //not necessary to handle all the error kinds, just the ones you care about
            //std::io::ErrorKind::PermissionDenied => todo!(),
            //std::io::ErrorKind::ConnectionRefused => todo!(),
            //std::io::ErrorKind::ConnectionReset => todo!(),
            ////std::io::ErrorKind::HostUnreachable => todo!(),
            ////std::io::ErrorKind::NetworkUnreachable => todo!(),
            //std::io::ErrorKind::ConnectionAborted => todo!(),
            //std::io::ErrorKind::NotConnected => todo!(),
            //std::io::ErrorKind::AddrInUse => todo!(),
            //std::io::ErrorKind::AddrNotAvailable => todo!(),
            ////std::io::ErrorKind::NetworkDown => todo!(),
            //std::io::ErrorKind::BrokenPipe => todo!(),
            //std::io::ErrorKind::AlreadyExists => todo!(),
            //std::io::ErrorKind::WouldBlock => todo!(),
            ////std::io::ErrorKind::NotADirectory => todo!(),
            ////std::io::ErrorKind::IsADirectory => todo!(),
            ////std::io::ErrorKind::DirectoryNotEmpty => todo!(),
            ////std::io::ErrorKind::ReadOnlyFilesystem => todo!(),
            ////std::io::ErrorKind::FilesystemLoop => todo!(),
            ////std::io::ErrorKind::StaleNetworkFileHandle => todo!(),
            //std::io::ErrorKind::InvalidInput => todo!(),
            //std::io::ErrorKind::InvalidData => todo!(),
            //std::io::ErrorKind::TimedOut => todo!(),
            //std::io::ErrorKind::WriteZero => todo!(),
            ////std::io::ErrorKind::StorageFull => todo!(),
            ////std::io::ErrorKind::NotSeekable => todo!(),
            ////std::io::ErrorKind::FilesystemQuotaExceeded => todo!(),
            ////std::io::ErrorKind::FileTooLarge => todo!(),
            ////std::io::ErrorKind::ResourceBusy => todo!(),
            ////std::io::ErrorKind::ExecutableFileBusy => todo!(),
            ////std::io::ErrorKind::Deadlock => todo!(),
            ////std::io::ErrorKind::CrossesDevices => todo!(),
            ////std::io::ErrorKind::TooManyLinks => todo!(),
            ////std::io::ErrorKind::InvalidFilename => todo!(),
            ////std::io::ErrorKind::ArgumentListTooLong => todo!(),
            //std::io::ErrorKind::Interrupted => todo!(),
            //std::io::ErrorKind::Unsupported => todo!(),
            //std::io::ErrorKind::UnexpectedEof => todo!(),
            //std::io::ErrorKind::OutOfMemory => todo!(),
            //std::io::ErrorKind::Other => todo!(),
            _ => println!("Error: {:#?}", e),
        }
    }

    //generic error handling
    let current_path = "./dir_does_not_exist";
    let files = list_files(current_path);
    match files {
        Ok(files) => {
            println!("Files in directory: {}", current_path);
            for file in files {
                println!("File: {} Size: {}", file.name, file.size);
            }
        }
        Err(e) => eprintln!("Generic Error Handling: {:#?}", e),
    }

    //anyhow error handling
    let anyhow_files = list_files_anyhow(current_path); 
    match anyhow_files {
        Ok(files) => {
            println!("Files in directory using anyhow: {}", current_path);
            for file in files {
                println!("File: {} Size: {}", file.name, file.size);
            }
        }
        Err(e) => eprintln!("Anyhow Error handling: {:#?}", e),
    }

    
    //thiserror error handling
    let file_not_found_filename = "data.json";
    let name = load_file_thiserror(file_not_found_filename);
    match name {
        Ok(name) => println!("Name: {}", name),
        Err(e) => eprintln!("Thiserror Error Handling: {:#?}", e),
    }

    let file_corrupted_filename = "corrupted_data.json";
    let name = load_file_thiserror(file_corrupted_filename);
    match name {
        Ok(name) => println!("Name: {}", name),
        Err(e) => eprintln!("Thiserror Error Handling: {:#?}", e),
    }
}
