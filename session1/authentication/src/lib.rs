pub fn greet_user(name: &str) -> String{
  format!("Hello, {}!", name)
}

#[derive(Debug, PartialEq)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LoginRole {
    Admin,
    User,
}

pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        User {
            username: username.to_lowercase(),
            password: password.to_string(),
            role,
        }
    }
}

pub fn get_users() -> [User; 2] {
    [
        User::new("admin", "password", LoginRole::Admin),
        User::new("mike", "password", LoginRole::User),
    ]
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {

    let users: [User; 2] = get_users(); 
    let username: String = username.to_lowercase();

    /*
    Searching for the User
    users.iter():
        Creates an iterator over the users array, where each item is a reference to a User (&User).
    .find(|user: &&User| user.username == username):
        The find method looks for the first User in the array where user.username == username.
        |user: &&User|: A closure that takes a reference to a reference (&&User) because the iterator produces &User.
    if let Some(user):
        If a matching user is found, the Some(user) branch is executed.
        The user variable is assigned the matching User.
    */
    if let Some(user) = users.iter().find(|user: &&User| user.username == username) {
        if user.password == password {
            return Some(LoginAction::Granted(user.role.clone()));
        } else {
            return Some(LoginAction::Denied);
        }
    }
    None
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
