use std::io;
use std::io::Write;

fn main() {

    loop {
        println!("(l) list entries | (a) add an entry | (d) delete an entry | (q) to quit.");
        print!("Please choose an option : ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Error reading the input.");

        if option.trim().to_lowercase() == "q" {
            break;
        } else if option.trim().to_lowercase() == "l" {
            list_entries();
        } else if option.trim().to_lowercase() == "a" {
            add_an_entry();
        } else if option.trim().to_lowercase() == "d" {
            delete_an_entry();
        } else {
            println!("Invalid input");
            continue;
        }
    }
}

fn list_entries() {
    println!("You chose to list all of the entries");
}

fn add_an_entry() {
    println!("You chose to add an entry");
}
fn delete_an_entry() {
    println!("You chose to delete an entry");
}
