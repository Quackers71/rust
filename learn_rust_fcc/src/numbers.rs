// Numbers

// No. 5 - Fix errors and panics to make it work
pub fn number() {
    let v1 = 251_u16 + 8;
    let v2 = i16::checked_add(251, 8).unwrap();
    println!("{},{}",v1,v2); // Output should be '259,259'
 }
 


// // No. 3 - Modify `assert_eq!` to make it work
// pub fn number() {
//     let x: u32 = 5; // added : u32 
//     assert_eq!("u32".to_string(), type_of(&x));

//     println!("Success!");
// }

// // Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
// fn type_of<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>())
// }
