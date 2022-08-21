// Functions - used to store block of code for re-use

pub fn run() {
    greeting("greet", "Q");

    //bind function values to variables
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // closure
    let num3 = 3; // using an outside variable
    let add_nums = |num1: i32, num2: i32| num1 + num2 + num3;
    println!("C Sum: {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}