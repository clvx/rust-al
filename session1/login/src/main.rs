use authentication::{login, read_line};

fn main() {
    let mut tries: i32 = 3;

    loop {
        println!("Enter a username: ");
        let username: String = read_line();
        println!("Enter a password: ");
        let password: String = read_line();
        if login(&username, &password) {
            println!("Login successful");
            break;
        } else {
            println!("Login failed");
            tries -= 1;
            if tries == 0 {
                println!("Too many failed attempts");
                break;
            }
        }
    }
}
