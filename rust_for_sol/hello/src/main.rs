// mod arrays;

fn main() {

    let i = 2;
    println!("i = {}", i);
    match i {
        0 => println!("0"),
        1 | 2 => println!("matched to either 1 or 2"),
        3..=4 => println!("matched to either 3 or 4"),
        _ => println!("default")
    }
    
}
