use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub fn hash_password(password: &str) -> String {
    use sha2::Digest;
    let mut hasher = sha2::Sha256::new();
    hasher.update(password);
    format!("{:X}", hasher.finalize()) // hexadecimal representation of a number
}

pub fn greet_user(name: &str) -> String{
  format!("Hello, {}!", name)
}

#[derive(Debug, PartialEq)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        User {
            username: username.to_lowercase(),
            password: hash_password(password),
            role,
        }
    }
}

/*
fn get_admin_users() {
    let _admin: Vec<String> = get_users().into_iter()
        .filter(|user| user.role == LoginRole::Admin)
        .map(|user| user.username)
        .collect();
}
*/

/*
 * get_users using vectors
pub fn get_users() -> Vec<User> {
    vec![
        User::new("admin", "password", LoginRole::Admin),
        User::new("mike", "password", LoginRole::User),
    ]
}
*/

pub fn get_default_users() -> HashMap<String, User> {
    let mut users: HashMap<String, User> = HashMap::new();
    users.insert("admin".to_string(), User::new("admin", "password", LoginRole::Admin));
    users.insert("mike".to_string(), User::new("mike", "password", LoginRole::User));
    users
}

pub fn save_users(users: HashMap<String, User>){
    let users_path: &Path = Path::new("users.json");
    let users_json = serde_json::to_string(&users).unwrap();
    std::fs::write(users_path, users_json).unwrap();
}

pub fn get_users() -> HashMap<String, User> {
    let users_path: &Path = Path::new("users.json");
    if users_path.exists() {
        //Load file
        let users_json = std::fs::read_to_string(users_path).unwrap();
        let users: HashMap<String, User> = serde_json::from_str(&users_json).unwrap();
        users
    } else {
        //Create file and returns it
        let users: HashMap<String, User> = get_default_users();
        let users_json = serde_json::to_string(&users).unwrap();
        std::fs::write(users_path, users_json).unwrap();
        users
    }
}

pub fn login(username: &str, password: &str) -> Option<LoginAction> {

    let username: String = username.to_lowercase();
    let password: String = hash_password(password); //shadowing password

    let users: HashMap<String, User> = get_users();
    if let Some(user) = users.get(&username) {
        if user.password == password {
            return Some(LoginAction::Granted(user.role.clone()));
        } else {
            return Some(LoginAction::Denied);
        }
    }


    /*
    /*
    let users: Vec<User> = get_users(); 
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
    */
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
