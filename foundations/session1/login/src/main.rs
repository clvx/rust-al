use authentication::{login, read_line};

fn main() {
    let mut tries: i32 = 3;

    loop {
        println!("Enter a username: ");
        let username: String = read_line();
        println!("Enter a password: ");
        let password: String = read_line();

        match login(&username, &password) {
           Some(authentication::LoginAction::Granted(login_role)) => {
               match login_role {
                   authentication::LoginRole::Admin => println!("Admin login successful"),
                   authentication::LoginRole::User => println!("User login successful"),
               }
               break;
           }
           Some(authentication::LoginAction::Denied) => {
               //Do nothing
           }
           None => {
               println!("Invalid username");
           }
        }

        println!("Login failed");
        tries -= 1;
        if tries == 0 {
            println!("Too many failed attempts");
            break;
        }
    }
}
