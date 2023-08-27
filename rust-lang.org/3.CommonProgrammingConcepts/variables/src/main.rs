fn main() {
    // Shadowing
    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    // println!("The value of x is: {x}");
    // x = 6;
    println!("The value of x is: {x}");

    let spaces = "            Yo";
    let _spaces = spaces.len();
    println!("{spaces}");
    println!("This string takes up {_spaces} characters");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("As a const - Three hours in seconds: {THREE_HOURS_IN_SECONDS}")
}
