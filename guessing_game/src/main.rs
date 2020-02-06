extern crate rand;

use std::io;
use std::cmp::Ordering; //cmp() method used for comparing
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .ok()
            .expect("failed to read line");


            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            }; //The parse() method on strings parses a string into some kind of number. Since it can parse a variety of numbers, we need to give Rust a hint as to the exact type of number we want.


        //Rust allows us to ‘shadow’ the previous guess with a new one. This is often used in this exact situation, where guess starts as a String, but we want to convert it to an u32. Shadowing lets us re-use the guess name, rather than forcing us to come up with two unique names like guess_str and guess, or something else.

            println!("You guessed: {}", guess);

            match guess.cmp(&secret_number) {
                Ordering::Less    => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal   => {
                    println!("You win!");
                    break;
            }
        }
    }
}
