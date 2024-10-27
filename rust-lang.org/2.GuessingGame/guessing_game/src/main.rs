use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    
    println!("Guess the number, between 1 & 100!");
    println!("Enter 'q' to quit");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_number}");
    let mut count = 0u32;

    loop {
        eprint!("Please input your guess : ");
        count +=1;

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() { //.expect("Please type a number!"); // Example of Shadowing
            Ok(num) => {
                if num < 1 || num > 100 {
                    eprintln!("Please enter a number between 1 and 100.");
                    continue;
                }
                num
            },
            Err(_) => {
                if guess.trim() == "q" {
                    count -=1;
                    println!("You took {count} guesses");
                    println!("But decided to quit game.");
                    break;
                } else {
                    eprintln!("Invalid Input, please type a number!");
                    continue;
                }
            }
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You win! the number was {secret_number}");
                println!("It took you {count} guesses");
                break;
            }
        }
    }
}