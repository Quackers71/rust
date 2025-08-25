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
