use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    
    println!("Please guess the secret number, between 1 & 100.");
    println!("*** You have 10 attempts *** Enter 'q' to quit");

    let secret_number = rand::thread_rng().gen_range(1..=100); 
    // println!("The secret number is: {secret_number}");
    let mut count = 0u32;
    const MAX_ATTEMPTS: u32 = 10;

    loop {
        count +=1;
        eprint!("Attempt {count}/{MAX_ATTEMPTS} - Please input your guess : ");

        let mut guess = String::new(); // creates a variable "guess" to store in the user input

        io::stdin() // calls the stdin function from the io module
            .read_line(&mut guess) // calls the read_line method, passing &mut guess as the argument - &mut makes the reference mutable
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() { // Example of Shadowing (re-using guess) //.trim() removes whitespace // .parse converts guess into an i32
            Ok(num) => { // enum variant Ok() - Successful Result
                if num < 1 || num > 100 {
                    eprintln!("Please enter a number between 1 and 100.");
                    continue;
                }
                num
            },
            Err(_) => { // enum variant Err{_} // _ catches all - If the parse is not able to convert the String into a Number
                if guess.trim().to_lowercase() == "q" {
                    // count +=1;
                    if count == 1 {
                        println!("You decided to quit the game.");
                    } else if count == 2 {
                        println!("You took {} legitimate guess", count - 1);
                        println!("But decided to quit the game.");
                    } else {
                        println!("You took {} legitimate guesses", count - 1);
                        println!("But decided to quit the game.");
                    }
                    break; // breaks loop & ends program
                } else {
                    eprintln!("Invalid Input, please type a number!");
                    // count +=1;
                    continue;
                }
            }
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) { // match expression with cmp call between guess and &secret_number (again a reference)
            Ordering::Less => println!("Too low!"), // Ordering type with enum variants (Less, Greater & Equal)
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                if count == 1 {
                    println!("What! You got the secret number in {count} guess...");
                    println!("Please go and buy a lottery ticket today!");
                } else {
                    println!("You win! the number was {secret_number}");
                    println!("It took you {count} legitimate guesses");
                }
                break; // breaks loop & ends program...
            }
        }

        if count >= MAX_ATTEMPTS {
            println!("GAME OVER: You've used all {MAX_ATTEMPTS} attempts.");
            println!("The secret number was : {secret_number}");
            break;
        }
    }
}