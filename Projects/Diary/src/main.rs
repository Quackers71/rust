use std::io;
use std::io::Write;
// use std::fs::File;
use std::fs::OpenOptions;
use std::io::Result; // Use io::Result for easier error handling in main

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
        .open("journal.log")?;
    // println!("File opened/created successfuly.");
    Ok(())
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
