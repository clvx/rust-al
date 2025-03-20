use authentication::{User, LoginRole, get_users, save_users};
use std::collections::HashMap;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command()]

struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]

enum Commands {
    /// List all users.
    List,
    /// Add a new user.
    Add{
        /// The user's login name
        username: String,
        /// The user's password (plaintext)
        password: String,
        /// Optional - mark as an admin
        #[arg(long)]
        admin: Option<bool>
    },
    /// Delete a user.
    Delete{
        /// The user's login name
        username: String,
    },
    /// Change a user's password
    ChangePassword{
        /// The user's login name
        username: String,
        /// The user's new password
        password: String,
    }
}

fn list_users() {
    println!("{:<20}{:<20}", "Username", "Role"); //padding of 20
    println!("{:-<40}", "");

    let users: HashMap<String, User> = get_users();
    users
        .iter()
        .for_each(|(_, user)|{
            println!("{:<20}{:20?}", user.username, user.role); //padding of 20
        })
}

fn add_users(username: String, password: String, admin: bool){
   let mut users: HashMap<String, User> = get_users();
   let role = if admin {LoginRole::Admin} else {LoginRole::User};

   let user = User::new(&username, &password, role);
   users.insert(username, user);
   save_users(users);
   println!("User added successfully");

}

fn delete_user(username: String){
    let mut users: HashMap<String, User> = get_users();
    if users.contains_key(&username){
        users.remove(&username);
        save_users(users);
        println!("User deleted successfully");
    } else {
        println!("User not found");
    }
}

fn update_password(username: String, password: String){
    let mut users: HashMap<String, User> = get_users();
    if users.contains_key(&username){
        let user = users.get_mut(&username).unwrap();
        user.password = authentication::hash_password(&password);
        save_users(users);
        println!("Password updated successfully");
    } else {
        println!("User not found");
    }
}

fn main() {
    /*
     * cargo run -- list 
     * cargo run -- --help
    */
    let cli = Args::parse();
    match cli.command{
        Some(Commands::List) => {
            //println!("List users here");
            list_users();
        }
        Some(Commands::Add{username, password, admin}) => {
            // As admin is an optional field, it's unwrapped with a default value of false
            add_users(username, password, admin.unwrap_or(false));
        }
        Some(Commands::Delete{username}) => {
            delete_user(username); 
        }
        Some(Commands::ChangePassword{username, password}) => {
            update_password(username, password);
        }
        None => {
            println!("Run with --help to see instructions.");
        }
    }
}
