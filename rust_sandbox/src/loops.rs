// Loops - used to iterate until a condition is met

pub fn run() {
    let mut count = 0;

    // infinite loop
    loop {
        count += 1;
        println!("Number: {}", count);

        if count == 3 {
            break;
        }
    }

    // while loop (FizzBuzz)
    while count <= 31 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count);
        }

        // increment
        count += 1;
    }

    // for range
    for x in 0..31 {
        if x % 15 == 0 {
            println!("x FizzBuzz");
        } else if x % 3 == 0 {
            println!("x Fizz");
        } else if x % 5 == 0 {
            println!("x Buzz");
        } else {
            println!("x: {}", x);
        }
    }
    
}