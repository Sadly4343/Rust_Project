//Rust Libary used for Read and Write of Functions
use std::io::{self, Write};

fn main() {
    let mut result: f64 = 0.0;
    //Calculator Loop Menu
    loop {
        //Print Line Function to Display to User
        println!("What would you like to calculate? ");
        println!("1. Addition ");
        println!("2. Multiplication ");
        println!("3. Division ");
        println!("4. Multiplication  ");
        println!("5. Factorial ");
        print!("6. Exit ");
        //Allows display above to return immediately rather than later.
        io::stdout().flush().unwrap();
        //Input is set to be changeable.
        let mut input: String = String::new();
        //Console Write
        io::stdin().read_line(&mut input).unwrap();
        // Input value trim and parses the value and ParseIntError in case of error.
        let input: Result<i32, std::num::ParseIntError> = input.trim().parse::<i32>();
        //Can compare the value returned to return error or correct value from user.
        match input {
            Ok(choice) if choice == 6 => {
                //Loop Exit on choice of 6
                println!("Exit");
                break;
            }
            Ok(choice) => {
                if !menu_choice(choice, &mut result) {
                    println!("Invalid Choice")
                }
            }
            //Error Handling for values not correct.
            Err(_) => {
                println!("That is not a valid number!");
            }
        }
    }
}
fn get_number_user_input() -> f64 {
    //Loop that runs until user inputs valid integer number.
    loop {
        print!("What is your number? ");
        io::stdout().flush().unwrap();
        let mut input: String = String::new();

        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid Number, not a number");
                continue;
            }
        }
    }
}
//Menu choices for the user to input
fn menu_choice(choice: i32, result: &mut f64) -> bool {
    match choice {
        1 => {
            //calls the addition function
            *result = addition(*result);
            println!("The result is: {}", *result);
            true
        }
        2 => {
            //Calls the subtraction function
            *result = subtraction(*result);
            println!("The result is: {}", *result);
            true
        }
        3 => {
            //Division
            *result = division(*result);
            println!("The result is: {}", *result);
            true
        }
        4 => {
            //Multiplication
            *result = multiplication(*result);
            println!("The result is: {}", *result);
            true
        }
        5 => {
            //Factorial
            let number = get_number_user_input();
            let result = factorial(number as u64);
            println!("The result is: {}", result);
            true
        }
        _ => false,
    }
}
//Addition function
fn addition(previous_result: f64) -> f64 {
    //Gets user inputs
    let first_number = get_number_user_input();
    let second_number = get_number_user_input();
    //Adds them together after and returns value.
    return first_number + second_number + previous_result;
}
//Subtraction function
fn subtraction(previous_result: f64) -> f64 {
    //User inputs
    let first_number = get_number_user_input();
    let second_number = get_number_user_input();
    //Subtract and return
    return first_number - second_number + previous_result;
}
//Division function
fn division(previous_result: f64) -> f64 {
    //User inputs
    let first_number = get_number_user_input();
    let second_number = get_number_user_input();
    //Returns values.
    return first_number / second_number + previous_result;
}
//Multiplication Function
fn multiplication(previous_result: f64) -> f64 {
    let first_number = get_number_user_input();
    let second_number = get_number_user_input();
    //Returns multiplication
    return first_number * second_number + previous_result;
}
//Factorial function
fn factorial(number: u64) -> u64 {
    //If number is larger than 20 do not complete factorial.
    if number > 20 {
        println!("Input too large, Numbers over 20 cannot be calculated. ");
        return 0;
    }
    //If number is == 1 or == 0 return 1 Base Case.
    if number == 1 || number == 0 {
        return 1;
    } else {
        //Return recurson of factorial to calculate number returned.
        return number * factorial(number - 1);
    }
}
