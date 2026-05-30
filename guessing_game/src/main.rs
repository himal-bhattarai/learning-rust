use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess the Number!");
    let secret_number = rand::thread_rng().gen_range(1..=100); // gnerates the random number between 1 to 100
    loop {
        println!("Enter your guess: ");

        let mut guess = String::new(); // storing the user input value

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line!");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        }; // parsing string to u32 number

        println!("you guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("you win 🎉!");
                break;
            },
        }
    }
}
