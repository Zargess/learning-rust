use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let target: i32 = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    println!("You guessed: {}", guess);
    println!("The correct value was: {}", target);
}
