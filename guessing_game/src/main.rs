use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;

fn main() {
    println!("Guess The Number!");

    loop {
        let secret_number: i32 = rand::thread_rng().gen_range(1, 101);

        println!("Please input your guess.");
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        println!("Secret number is: {}", secret_number);


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Greater => println!("{}","Too Big".red()),
            Ordering::Equal => {
                println!("{}","You Win!".green());
                break;
            }
        }

    }
}
