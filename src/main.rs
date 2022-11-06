use std::{io, cmp::Ordering};
use guessing_game::Guess;

fn main() {
    println!("Guess the number between 1 and 100!");

    let secret_number = Guess::get_secret_number();

    // println!("The secret number is: {secret_number}");
    let mut tries: u32 = 0;
    loop {

        println!("Please input your guess.");

        let mut guess = String::new();
        

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        
            let guess: Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        tries += 1;
        println!("You guessed: {}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congratulations! You WIN!");
                println!("\nTries: {tries}");
                break;
            }
        }
    }
}
