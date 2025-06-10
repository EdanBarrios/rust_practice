use std::io;
use std::cmp::Ordering; 
use rand::Rng;
use colored::Colorize;

fn main() {
    println!("Welcome to the Guessing Game!");
    println!("I have selected a random number between 1 and 100.");
    println!("Try to guess it!");
    let secret_number =  rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop{
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too low!".red()),
            Ordering::Greater => println!("{}", "Too high!".green()),
            Ordering::Equal => {
                println!("Congratulations, you guessed it! \nGame Over!");
                break;
            }
        }
    }
}