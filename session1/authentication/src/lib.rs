pub fn greet_user(name: &str) -> String{
  format!("Hello, {}!", name)
}

#[derive(Debug, PartialEq)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(Debug, PartialEq)]
pub enum LoginRole {
    Admin,
    User,
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username: String = username.to_lowercase();

    if username != "admin" && username != "mike" {
        return None;
    }

    if username == "admin" && password == "password" {
        Some(LoginAction::Granted(LoginRole::Admin))
    } else if username == "mike" && password == "password" {
        Some(LoginAction::Granted(LoginRole::User))
    } else {
        Some(LoginAction::Denied)
    }
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
        assert_eq!(Some(LoginAction::Granted(LoginRole::Admin)), login("ADMIN", "password"));
        assert_eq!(Some(LoginAction::Granted(LoginRole::Admin)), login("admin", "password"));
        assert_eq!(Some(LoginAction::Granted(LoginRole::User)), login("mike", "password"));
        assert_eq!(Some(LoginAction::Denied), login("admin", "wrong_password"));
        assert_eq!(None, login("wrong_username", "password"));
    }
}
