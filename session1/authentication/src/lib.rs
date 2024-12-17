pub fn greet_user(name: &str) -> String{
  format!("Hello, {}!", name)
}

pub fn login(username: &str, password: &str) -> bool {
  username == "admin" && password == "password"
}

// read_line reads a line from the standard input and returns an input
pub fn read_line () -> String {
    let mut input: String = String::new(); //input buffer
    std::io::stdin().read_line(&mut input).expect("Stdin not working"); //read line from stdin and
                                                                        //prints error message if
                                                                        //it fails
    input.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user() {
        assert_eq!("Hello, Michael!", greet_user("Michael"));
    }

    #[test]
    fn test_login() {
        assert_eq!(true, login("admin", "password"));
        assert_eq!(true, login("admin", "password"));
        assert_eq!(false, login("admin", "wrong_password"));
        assert_eq!(false, login("wrong_username", "password"));
    }
}
