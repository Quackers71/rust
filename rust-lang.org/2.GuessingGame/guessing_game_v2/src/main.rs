use std::io;
// use std::cmp::Ordering; // apparently unused
use rand::Rng;

// The below code was taken from AI and works like the original guessing gane version ;-)

fn main() {
    println!("Welcome to the Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess_count = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        guess_count += 1;
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number)
 {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => {
                println!("You   
 win in {} guesses!", guess_count);
                break;
            }
        }
    }
}