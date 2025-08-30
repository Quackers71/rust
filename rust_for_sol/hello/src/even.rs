pub fn is_even(num: u8) -> &'static str { 
    if num % 2 == 0 { "even" } else { "odd" }
}

/* pub fn is_even(num: u8) -> bool {
    let digit: u8 = num % 2;
    digit == 0
} */