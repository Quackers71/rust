pub fn run() {
    // print to console
    println!("Hello from the print.rs file");

    // basic formatting
    println!("Project #{}", 1);

    println!("{} is really {}", "Q", "CrazyCryptoQ");

    // positional args
    println!("{0} is from {1} & {0} likes to {2}", "Q", "Hams", "play T&P" );

    // name args
    println!(
    "{name} likes to play {activity}",
    name="Q",
    activity="T&P"
    );

    // placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal {:o}", 10, 10, 10);

    // placeholder for debug trait - tuple
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}