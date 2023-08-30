fn main() {
    /* let number = 3;

    if number < 5 {
        println!("condition was : true");
    } else {
        println!("condition was : false");
    } */

    // Handling Multiple Conditions with else if
    /* let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using if in a let Statement
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}"); // 5 */

    // Returning Values from Loops
    /* let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("{counter} count");

        if counter == 10 {
            println!("Counter : {counter}");
            break counter * 2;
        }
    };

    println!("The counter * 2 is now : {result}"); */

    // Loop Labels to Disambiguate Between Multiple Loops
    /* let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}"); */

    // Conditional Loops with while
    /* let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!"); */

    // Looping Through a Collection with for
    /* let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the index is : {index} and the value is: {}", a[index]);

        index += 1;
    } */

    // a more concise alternative
    /* let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    } */

    // using for in with .rev() - reverse the range
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}
