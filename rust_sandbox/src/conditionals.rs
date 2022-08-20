// Conditionals - used to check the condition of something and act on the result

pub fn run() {
    let age = 18;
    let check_id: bool = true;
    let knows_person_of_age = true;

    // if/else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry Sir, but you are under age, to be able to drink in this establishment!");
    } else {
        println!("Bartender: I will need to see your ID...");
    }

    // shorthand if
    let is_of_age = if age >= 21{true} else {false};
    println!("Is of Age: {}", is_of_age);
}