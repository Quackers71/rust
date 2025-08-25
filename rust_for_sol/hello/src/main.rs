// mod arrays;

fn main() {

    // if, else if, else
    let n = 3;
    println!("The number is {}", n);
    if n > 0 {
        println!("which is greater than 0");
    } else if n < 0 {
        println!("which is less than 0");
    } else {
        println!("which is 0");
    }

    // for loop
    println!("for loop 1..6");
    for i in 1..6 {
        println!("{},", i);
    }

    // while loop
    println!("while loop - starts at 1 and < 6 with a break at 4");
    let mut l = 1;
    while l < 6 {
        println!("{}", l);
        l += 1;
        if l == 4 {
            println!("exit");
            break
        }
    }
}
