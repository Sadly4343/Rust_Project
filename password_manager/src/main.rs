//Hash crate
use bcrypt::{hash, verify, DEFAULT_COST};
//For Json Writing and Reading
use serde::{Deserialize, Serialize};
use serde_json;
//Used for Hashmaps
use std::collections::HashMap;
//File options
use std::fs::{File, OpenOptions};
//Read and write inputs from user
use std::io::{self, Write};
//Path for files
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
//Structure for UserInfo Similiar to Objects in OOP
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
    //Verify if password is correct to allow user to get information.
    fn verify_password(&self, ent_password: &str) -> bool {
        verify(ent_password, &self.hashed_pass).unwrap_or(false)
    }
}
//Function for PasswordManager allows loading of passwords from file.
impl PasswordManager {
    //Function to load passwords from JSON File
    fn load_passwords(&mut self, file_path: &str) -> std::io::Result<()> {
        if Path::new(file_path).exists() {
            //File open
            let file = File::open(file_path)?;
            //Accounts using Serde_Json library to convert.
            let accounts: HashMap<String, UserInfo> = serde_json::from_reader(file)?;
            self.accounts = accounts;
        }
        Ok(())
    }
}
//Main Function Loop for Program
fn main() {
    //Allows passwordmanager to be mutable or changeable into a new HashMap
    let mut passwordmanager = PasswordManager {
        accounts: HashMap::new(),
    };
    //Assigns File_Path to the Json File with store information.
    let file_path = r"password_storage.json";
    //Matches and compares the filepath with the Load_Passwords function
    match passwordmanager.load_passwords(file_path) {
        //Successfully loaded passwords.
        Ok(_) => {
            println!("Passwords loaded");
        }
        //Failed to load passwords.
        Err(e) => eprintln!("Failed to load passwords: {}", e),
    }
    //Program loop for password manager
    loop {
        println!("Welcome to your password manager!");
        println!("");
        println!("1. Find your information!");
        println!("2. Create an Account!");
        println!("3. Select 3 to Exit. ");
        //Pushes displaying lines first directly ahead of input.
        io::stdout().flush().unwrap();
        //Mutable input is a new string
        let mut input: String = String::new();
        //Reads the line to understand the Input and store it.
        io::stdin().read_line(&mut input).unwrap();
        //Attemps to parse input into an integer from string and deals with error handling.
        let input: std::result::Result<i32, std::num::ParseIntError> = input.trim().parse::<i32>();
        //Compare inputs and match choice of input
        match input {
            //If 4 is chosen break and end loop and program.
            Ok(choice) if choice == 4 => {
                println!("Exiting");
                break;
            }
            //If menu choice is not valid return error.
            Ok(choice) => {
                if !menu_choice(choice, &mut passwordmanager) {
                    println!("Invalid Choice")
                }
            }
            //Return error if choice is not correct.
            Err(_) => {
                println!("That is not valid");
            }
        }
    }
}
//Retrieves user input for functions helper function
fn get_user_input(input: &str) -> String {
    //Loop that runs until user inputs valid integer number.
    loop {
        //Asks for input
        print!("What is your {}? ", input);
        io::stdout().flush().unwrap();
        let mut input: String = String::new();
        //Reads line of input
        io::stdin().read_line(&mut input).unwrap();
        //Trims input of whitespace
        let trimmed = input.trim();
        //If empty returns input cannot be empty.
        if !trimmed.is_empty() {
            return trimmed.to_string();
        } else {
            println!("Enter an input cannot be empty! ")
        }
    }
}
//Menu choices
fn menu_choice(choice: i32, passwordmanager: &mut PasswordManager) -> bool {
    //Matches choices to integer values.
    match choice {
        1 => {
            //Calls Find_password function
            find_password(&passwordmanager.accounts);
            true
        }
        2 => {
            //If let to call create_account and fails if doesnt work.
            if let Err(_e) = create_account(&mut passwordmanager.accounts) {
                eprintln!("Failed to create account");
            }
            true
        }
        //Returns if not the correct choice.
        _ => false,
    }
}
//Used to find password and uses user inputs of password and username.
fn find_password(accounts: &HashMap<String, UserInfo>) {
    let username = get_user_input("username");
    let password = get_user_input("password");
    println!("you enterered{}", username);
    println!("you enterered{}", password);
    println!("Accounts: {:?}", accounts);
    //Compares and finds values within accounts that match username.
    if let Some(user_info) = accounts.get(&username) {
        //Calls verify password to check if password works and is the same hashed.
        if user_info.verify_password(&password) {
            println!("Password verified for {}", username);
            println!("Email: {}", user_info.email);
            println!("Two Factor: {}", user_info.two_factor);
        } else {
            println!("Incorrect try again! ");
        }
    } else {
        println!("Invalid account try again! ");
    }
}
//Creates new account to the file.
fn create_account(accounts: &mut HashMap<String, UserInfo>) -> io::Result<()> {
    let username1 = get_user_input("Choose your username! ");
    let password1 = get_user_input("Choose your account password ");
    let email1 = get_user_input("Enter your email here ");
    //Hashed password.
    let hashed_pass1: String = hash(&password1, DEFAULT_COST).unwrap();
    //Assigns values from Userinfo structure
    let new_user = UserInfo {
        username: username1,
        email: email1,
        hashed_pass: hashed_pass1,
        two_factor: false,
    };
    //Mutable current_accounts of previous accounts in JSON File.
    let mut current_accounts = if Path::new("password_storage.json").exists() {
        let file = File::open("password_storage.json")?;
        serde_json::from_reader(file).unwrap_or_else(|_| HashMap::new())
    }
    //If empty new HashMap
    else {
        HashMap::new()
    };
    //Inserts new account into the current_accounts
    current_accounts.insert(new_user.username.clone(), new_user);
    //Reads file and writes to the file the new information updated.
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("password_storage.json")?;
    //serdeJSON library to convert
    serde_json::to_writer(file, &current_accounts)?;

    Ok(())
}
