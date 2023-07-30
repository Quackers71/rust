// Char, Bool and Unit
// No.1 - Unit Type 

// Make it work, don't modify `implicitly_ret_unit` !
pub fn unit() {
    let _v: () = (); // tuple

    let v: (i32, i32) = (2, 3);
    assert_eq!(_v, implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
// fn explicitly_ret_unit() -> () {
//     println!("I will return a ()");
// }
