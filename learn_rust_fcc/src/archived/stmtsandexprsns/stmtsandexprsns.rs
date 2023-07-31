// Statements and Expressions
// Examples

pub fn express() {
    let x = 5u32;

    // let y is a variable assignment and classed as a Statement
    let y: u32 = {
        // all of the code inside {} is classed as an Expression because there is a result and value...
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y` because the ; has been omitted from the line below
        x_cube + x_squared + x
    };

    let z: u32 = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);

}
