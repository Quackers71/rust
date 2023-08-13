// Char, Bool and Unit
// No.2 - Char = '' & String = ""

// Make it work
pub fn specific_char() {
    let c1 = '中';
    print_char(c1);

    println!("Success!")
} 

fn print_char(c : char) {
    println!("   {}", c);
}
