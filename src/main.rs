use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guess the namber!");

    let secret: u32 = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret}");
    
    println!("Please input your guess.");

    let mut guess_str: String = String::new();

    io::stdin()
    .read_line(&mut guess_str)
    .expect("Failed to read line");

    println!("You guessed: {}", guess_str);

    let mut guess: u32 = guess_str.trim().parse().expect("Please type a number!");


    match guess.cmp(&secret){
        Ordering::Less => println!("Too Small!"),
        Ordering::Greater => println!("Too Big"),
        Ordering::Equal => println!("You Win"),
    }

}