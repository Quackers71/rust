// Strings and String Slices

pub fn run() {
    // let person_name_slice = "Donald Mallard";
    // let person_name_string = person_name_slice.to_string();

    let person_name_string = String::from("Donald Mallard");
    let person_name_slice = &person_name_string[6..];
    let person_name_slice2 = person_name_string.as_str();

    println!("String : {}", person_name_string);
    println!("Slice : {}", person_name_slice);
    println!("Slice2 : {}", person_name_slice2);
}