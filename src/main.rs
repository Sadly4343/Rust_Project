//Rust Libary used for Read and Write of Functions
use std::io::{self, Write};

fn main() {
    //Calculator Loop Menu
    loop {
        //Print Line Function to Display to User
        println!("Enter your calculation or type Exit to leave.");
        //Allows display above to return immediately rather than later.
        io::stdout().flush().unwrap();
        //Input is set to be changeable.
        let mut input = String::new();
        //Console Write
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim().parse::<i32>();

        match input {
            Ok(num) => println!("Youve entered: {}", num),
            Err(_) => println!("That is not a valid number!"),
        }
    }
}
