// mod arrays;

fn main() {
    let name = String::from("Peregrine Falcon"); // define the name ("Raven") and type String
    let bird =  Bird {name, attack: 10}; // create instance of your struct = Struct Type {define types for all fields}
    bird.print_details(); // call the instance and method form the struct
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
