use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guessing game");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Input the guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            // expect crashes the program with panic!
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
