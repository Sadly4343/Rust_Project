use bcrypt::{hash, verify, DEFAULT_COST};
use std::collections::HashMap;
use std::io::{self, stdout, Write};
use serde::{Serialize, Deserialize}


#[derive(Serialize, Deserialize)]
struct UserInfo {
    username: String,
    email: String,
    hashed_pass: String,
    two_factor: bool,
}
//structure converted into hashmap for storage and search
struct PasswordManager {
    accounts: HashMap<String, UserInfo>,
}
//function and methods used for the struct UserInfo
impl UserInfo {
    //Create and Hash password for account
    fn crypt_password(password: &str) -> String {
        let hashed_pass = hash(password, DEFAULT_COST).unwrap();

        hashed_pass
    }
    //Verify if password is correct to allow user to get information.
    fn verify_password(&self, ent_password: &str) -> bool {
        verify(ent_password, &self.hashed_pass).unwrap_or(false)
    }
}

fn main() {

    let mut passwordmanager = PasswordManager {
        accounts: HashMap::new(),
    };

    accounts.insert(
        "John_doe".to_string(),
        UserInfo::new("John_doe","john@example.com", )
    )

    loop {
        println!("Welcome to your password manager!");
        println!("");
        println!("1. Find your password!");
        println!("2. Change your password!");
        println!("3. Create new password");
        println!("4. Two-Factor Authentication");
        println!("5. Delete an account password");
        println!("6. Select 6 to Exit. ");

        io::stdout().flush().unwrap();

        let mut input: String = String::new();

        io::stdin().read_line(&mut input).unwrap();

        let input: Result<i32, std::num::ParseIntError> = input.trim().parse::<i32>();

        match input {
            Ok(choice) if choice == 6 => {
                println!("Exiting");
                break;
            }
            Ok(choice) => {
                if !menu_choice(choice) {
                    println!("Invalid Choice")
                }
            }
            Err(_) => {
                println!("That is not valid");
            }
        }
    }
}
fn get_user_input(input: &str) -> String {
    //Loop that runs until user inputs valid integer number.
    loop {
        print!("What is your {}? ", input);
        io::stdout().flush().unwrap();
        let mut input: String = String::new();

        io::stdin().read_line(&mut input).unwrap();

        let trimmed = input.trim();

        if !trimmed.is_empty() {
            return trimmed.to_string();
        } else {
            println!("Enter an input cannot be empty! ")
        }
    }
}

fn menu_choice(choice: i32) -> bool {
    match choice {
        1 => {
            find_password();
            true
        }
        2 => {
            println!("yes");
            true
        }
        3 => {
            println!("yes");
            true
        }
        4 => {
            println!("yes");
            true
        }
        5 => {
            println!("yes");
            true
        }
        _ => false,
    }
}

fn find_password(accounts: &HashMap<String, UserInfo>) {
    let username = get_user_input("username");
    let password = get_user_input("password");
    println!("you enterered{}", username);
    println!("you enterered{}", password);
    
    if let Some(user_info) = accounts.get(&username) {
        if user_info.verify_password(&password) {
            println!("Password verified for {}", username);
        }   else{
            println!("Incorrect try again! ");
        }
        
    } else {
        println!("Invalid account try again! ");
    }
}

fn create_account(accounts: &mut HashMap<String, UserInfo>) {
    let username = get_user_input("Choose your username! ");
    let password = get_user_input("Choose your account password ");


}