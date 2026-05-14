use std::cmp::Ordering;
use std::io;
use rand::{RngExt};
use colored::*;

struct Guess {
    num: u32,
}

impl Guess {
    fn new(num_s: String) -> Result<Guess, String> {
        let num: u32 = match num_s.trim().parse() {
            Ok(num) => num,
            Err(_) => { return Err("Please Enter a valid number".to_string()) },
        };
        if num < 1 || num > 100 {
            return Err("Number should be in between 1 and 100".to_string());
        }
        Ok(Guess { num })
    }
}

fn main() {
    println!("Guess the Number: ");

    println!("Please Input your guess: ");

    let secret_number = rand::rng().random_range(1..101);
    // println!("The secret number is: {}", secret_number);
    let mut attempts: i32 = 0;

    loop {
        attempts += 1;
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess = match Guess::new(guess) {
            Ok(guess) => guess.num,
            Err(msg) => { println!("{}", msg); continue; },
        };

        match guess.cmp(&secret_number)  {
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Greater => println!("{}","Too big!".red()),
            Ordering::Equal => { let message = format!("You win! after {} attempts", attempts);
                println!("{}", message.green());
                break;
            },
        }
    }
}
