use std::io;

fn main() {
    println!("Hello! Guess the number.");
    println!("Please input your guess:");

    let mut guess = String::new(); // mut - mutable variable

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
