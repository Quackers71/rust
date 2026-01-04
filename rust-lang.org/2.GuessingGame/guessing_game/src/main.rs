use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    
    println!("{}","Please guess the secret number, between 1 & 100.".yellow());
    println!("{}",format!("*** You have {MAX_ATTEMPTS} attempts *** Enter 'q' to quit").on_bright_blue());

    let secret_number = rand::rng().random_range(1..=100);
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
                    eprintln!("{}","Please enter a number between 1 and 100.".blue());
                    // check for game over b4 continuing
                    if count >= MAX_ATTEMPTS { break; }
                    continue;
                }
                num
            },
            Err(_) => { // enum variant Err{_} // _ catches all - If the parse is not able to convert the String into a Number
                if guess.trim().to_lowercase() == "q" {

                    if count == 1 {
                        println!("{}","You decided to quit the game.".blue());
                    } else if count == 2 {
                        println!("You took {} legitimate guess", count - 1);
                        println!("{}","But decided to quit the game.".blue());
                    } else {
                        println!("You took {} legitimate guesses", count - 1);
                        println!("{}","But decided to quit the game.".blue());
                    }
                    break; // breaks loop & ends program
                } else {
                    eprintln!("{}","Invalid Input, please type a number!".on_red());

                    if count >= MAX_ATTEMPTS {
                        println!("{} {}", "GAME OVER: ".on_red().bold(),format!("You've used all {MAX_ATTEMPTS} attempts.").red());
                        println!("{}",format!("The secret number was : {secret_number}").blue().bold());
                        break;
                    }
                    continue;
                }
            }
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) { // match expression with cmp call between guess and &secret_number (again a reference)
            Ordering::Less => println!("{}", "Too low!".red()), // Ordering type with enum variants (Less, Greater & Equal)
            Ordering::Greater => println!("{}", "Too high!".red()),
            Ordering::Equal => {
                if count == 1 {
                    println!("{}",format!("What! You got the secret number in {count} guess...").green().bold());
                    println!("{}","Please go and buy a lottery ticket today!".yellow());
                } else {
                    println!("{} {}", "You win! the number was ".green().bold(),format!("{secret_number}").green().bold());
                    println!("{}",format!("It took you {count} legitimate guesses").blue().bold());
                }
                break; // breaks loop & ends program...
            }
        }

        if count >= MAX_ATTEMPTS {
            println!("{} {}", "GAME OVER: ".on_red().bold(),format!("You've used all {MAX_ATTEMPTS} attempts.").red());
            println!("{}",format!("The secret number was : {secret_number}").blue().bold());
            break;
        }
    }
}