//Rust Libary used for Read and Write of Functions
use std::io::{self, Write};

fn main() {
    //Calculator Loop Menu
    loop {
        //Print Line Function to Display to User
        println!("Enter your calculation or type Exit to leave.");
        //
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        println!("You entered: {}", input);
    }
}
