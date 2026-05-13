use std::cmp::Ordering;
use std::io;
use rand::{RngExt};
use colored::*;

fn main() {
    println!("Guess the Number: ");

    println!("Please Input your guess: ");

    let secret_number = rand::rng().random_range(1..101);

    loop {
        println!("The secret number is: {}", secret_number);

        let mut guess = String::new();



        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number)  {
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Greater => println!("{}","Too big!".red()),
            Ordering::Equal => { println!("{}","You win!".green()); break;},
        }
    }
}
