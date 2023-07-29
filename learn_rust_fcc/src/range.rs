// Numbers

// No. 9 - Range (ascii code 97-122)
pub fn range() {
    let mut sum: i32 = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}",c);
        println!("{}",c as i8); // outputs ascii code for chars from a to z
    }
}