use rand::Rng;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);


    let mut guess = String::new();
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    if guess != "" {
        hello()
    }

    println!("You guessed: {}", guess);
}

fn hello() {
    println!("Hello, world!");
}
