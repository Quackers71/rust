// Vectors - Resizable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // get single value
    println!("Single Value: {}", numbers[0]);

    // re-assign a value
    numbers[2] = 30;

    // add on to vector
    numbers.push(6);
    numbers.push(7);

    println!("{:?}", numbers);

    // get vector length
    println!("Vector length: {}", numbers.len());

    // pop off the last value
    numbers.pop();

    println!("{:?}", numbers);

    // vectors are stack allocated
    println!("The vector occupies {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // loop through vectors values
    for x in numbers.iter() {
        println!("Number {0}: {1}", numbers.len(), x);
    }

    // loop & mutate values
    for x in numbers.iter_mut() {
        *x *=2;
    }

    println!("Numbers Vec: {:?}", numbers);
}