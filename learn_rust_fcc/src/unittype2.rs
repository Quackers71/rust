// Char, Bool and Unit
// No.2 - Unit Type 

// Modify `4` in assert to make it work
use std::mem::size_of_val;
pub fn unit() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);
    println!("{}",size_of_val(&unit)); // 0

    println!("Success!");
}
