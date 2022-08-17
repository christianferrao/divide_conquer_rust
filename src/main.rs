/*
NUMBER GUESSER -> The algorithm will be able to guess the number the person thought about.

The strategy is to stabelish a initial step till it gets something in between
*/

use std::io;
use rand::Rng;

fn response_wrapper() -> String {
    let mut buffer = String::new();
    loop {
        let input = match io::stdin().read_line(&mut buffer) {
            Ok(_) => break,
            Err(error) => {
                println!("Invalid response! Enter a valid response! {}", error);
                continue;
            }
        };
    }
    return buffer;
}

fn possible_inputs() -> String {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input enter b, s or =");
        match input.trim() {
            "b" | "s" | "="  => return input,
            _ => println!("Wrong input, enter b, s or ="),
        }
    } 
}


fn number_guesser() {
    println!("Think about a number between 0 and 10000...");

    println!("Are you ready? Enter any letter....");

    let mut guess_range = rand::thread_rng();
    let mut new_guess = guess_range.gen_range(0..10001);
    let mut smallest_limit = 0;
    let mut biggert_limit = 10000;
    loop {
        println!("Is the number you chose smaller or bigger than {}", new_guess); 
        let mut user_input = possible_inputs(); 
        if user_input.trim() == "s" {
            // smallest_limit 
            biggert_limit = new_guess;
            new_guess = new_guess - (biggert_limit - smallest_limit) / 2;
            continue;
        } else if user_input.trim() == "b" {
            smallest_limit = new_guess;
            new_guess = new_guess + (biggert_limit - smallest_limit) / 2;
            continue;
        } else if user_input.trim() == "=" {
            print!("Great! the number is {}", new_guess);
            break;
        }
    }

}

fn main() {
    // let response = possible_inputs();
    // println!("{}, possible_inputs erpepe", response)
    number_guesser();
}
