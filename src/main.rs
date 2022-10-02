use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Guess the number between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");
    let mut tries: u32 = 0;
    loop {

        println!("Please input your guess.");

        let mut guess = String::new();
        

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        tries += 1;
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
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
