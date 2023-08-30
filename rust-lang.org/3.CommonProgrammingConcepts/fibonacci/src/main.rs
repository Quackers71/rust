fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

fn main() {
    let mut i = 0;

    loop {
        println!("The {}th Fibonacci number is: {}", i, fibonacci(i));
        i += 1;

        if i == 11 {
            break;
        }
    }
}
