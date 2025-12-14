use std::io;
use std::io::Write;
// use std::fs::File;
use std::fs::OpenOptions;
use std::io::Result; // Use io::Result for easier error handling in main

const JOURNAL_FILE: &str = "journal.log";

fn main() {

    let _ = create_journal();

    loop {
        println!("(l) list entries | (a) add an entry | (d) delete an entry | (q) to quit.");
        print!("Please choose an option : ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Error reading the input.");

        if option.trim().to_lowercase() == "q" {
            println!("You ended the program!");
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

fn create_journal() -> Result<()> {

    let _file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true) // create file if it doesn't exist
        .open(JOURNAL_FILE)?;
    // println!("File opened/created successfuly.");
    Ok(())
}

fn list_entries() -> Result<()> {
    println!("You chose to list all of the entries");

    let _file = OpenOptions::new()
        .read(true)
        .open(JOURNAL_FILE)?;

    println!("");
    println!("--- Contents of the Journal ---");
    let contents = std::fs::read_to_string(JOURNAL_FILE)?;
    println!("{}", contents);
    println!("---   End of the Contents   ---");
    println!("");
    Ok(())
}

fn add_an_entry() {
    println!("");
    println!("You chose to add an entry");
    println!("");
}
fn delete_an_entry() {
    println!("");
    println!("You chose to delete an entry");
    println!("");
}
