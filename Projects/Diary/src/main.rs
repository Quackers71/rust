use std::fs::{OpenOptions, File};
use std::io::{self, Write, Result, BufRead, BufReader}; // Use io::Result for easier error handling in main
use chrono::Local;

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
            let _ = list_entries();
        } else if option.trim().to_lowercase() == "a" {
            if let Err(e) = add_an_entry() {
                eprintln!("An error occurered: {}", e);
            }
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

fn add_an_entry() -> Result<()> {

    let mut user_input = String::new();

    println!("");
    print!("Please add your entry : ");
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut user_input)?;

    let entry_to_write = user_input.trim();
    if entry_to_write.is_empty() {
        println!("Error: No entry added");
        return Ok(()); // Exit function immediately so nothing is written
    }

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let id = get_next_id()?;

    let mut file = OpenOptions::new()
        .append(true) // open in append mode
        .create(true) // create file if it doesn't exist
        .open(JOURNAL_FILE)?;

    writeln!(&mut file, "[{}] [{}] - {}", id, timestamp, entry_to_write)?;
    println!("You successfully added entry no. {} to the {}", id, JOURNAL_FILE);
    Ok(())
}

// Helper function to calculate the next ID
fn get_next_id() -> Result<usize> {
    if !std::path::Path::new(JOURNAL_FILE).exists() {
        return Ok(1);
    }
    let file = File::open(JOURNAL_FILE)?;
    let reader = BufReader::new(file);
    // counting lines as the ID (i.e. if 5 lines exist, next ID is 6)
    let count = reader.lines().count();
    Ok(count + 1)
}

fn delete_an_entry() {
    println!("");
    println!("You chose to delete an entry");
    println!("");
}
