pub fn tuple() {
    let tuple: (u8, bool, f32) = (5, true, 2.1);
    let tuple2 = (3, 5);

    // print structure of array & other objects
    println!("first {}, second {}, third {}", tuple.0, tuple.1, tuple.2);
    println!("{:?}", tuple2);

    let (a, b, c) = tuple; // extracts out the parameters into the tuple object

    // destructuring
    println!("first {}, second {}, third {}", a, b, c);
}