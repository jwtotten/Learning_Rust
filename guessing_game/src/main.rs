use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hello! Guess the number.");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Secret number = {secret_number}");

    println!("Please input your guess:");

    let mut guess = String::new(); // mut - mutable variable

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    };
}
