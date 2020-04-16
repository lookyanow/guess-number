use std::io;
use rand::Rng;

fn main() {
    println!("Guess game programm");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Secret number: {}", secret_number);

    println!("Enter your number");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed: {}", guess);
}
