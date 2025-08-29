fn main() {

    /* let a = 10;
        let b = 15;
        println!("Hello, Rust!, a= {} & b= {}", a, b); */

        /* tuples::tuple();
        
        let num = 10;
        println!("your number {} is {}", num, even::is_even(num)); */

    //  muts::muta();

    // arrays::borrowing_slice();

    /*    let arr = [0, 1, 2, 3]; // length
    let slice = &arr[1 .. 3]; // [1, 2] don't know the length

    arrays::borrowing_slice(arr, slice); */

   let str: &str = "Hello Rust!";
   let mut string: String = String::from("Hello Rust");

   let slice = &string[.. 6];
   slice.len();

   string.push_str(" ");
   string.push('1');
   string.push_str("! Bob");
   string = string.replace("Hello", "Bye");

   println!("{}", string);

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

   let i = 2;
    println!("i = {}", i);
    match i {
        0 => println!("0"),
        1 | 2 => println!("matched to either 1 or 2"),
        3..=4 => println!("matched to either 3 or 4"),
        _ => println!("default")
    }
}

// main and functions from structs and traits
// mod arrays;

fn main() {
    let name = String::from("Peregrine Falcon"); // define the name ("Peregrine Falcon") and type String
    let bird =  Bird {name, attack: 10}; // create instance of your struct = Struct Type {define types for all fields}
    bird.print_details(); // call the instance and method form the struct

    println!("The bird can fly : {}", bird.can_fly());
    println!("The bird is an animal : {}", bird.is_animal());
}

// struct similar to classes
struct Bird {
    name: String,
    attack: u64
}

// to implement a method to your struct (class)
impl Bird {
    fn print_details(&self) {
        println!("The type of bird is a {}", self.name);
        println!("It's attack value is {}", self.attack);
    } 
}

impl Animal for Bird {
    fn can_fly(&self) -> bool {
        true
    }
}

// Rust does not support inheritance where you extend a class (OOP), it only supports interfaces (traits)
trait Animal {
    fn can_fly(&self) -> bool;
    fn is_animal(&self) -> bool {
        true
    }
}

// enums are objects that represent a certain value

fn main() {
    let a: MyEnum = MyEnum::A;
    let b: MyEnum = MyEnum::B(5); 
    let c: MyEnum = MyEnum::C{x:10, y:20}; 
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
    
    if let MyEnum::B(val) = b {
        println!("{}", val);
    }

    if let MyEnum::C{x, y} = c {
        println!("{} {}",x, y);
    }
}

#[derive(Debug)]
enum MyEnum {
    A,
    B(i32),
    C {x: i32, y: i32}
}