// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
pub fn shadow() {
    let x: i32 = 5;

    {
        let x = 12;
        assert_eq!(x, 12); // orginally 5
    }

    assert_eq!(x, 5); // originally 12

    let x: i32 = 42; // This wasn't explained as to why you can re-declare x !? immutable vs mutable
    println!("{}", x); // Prints "42".
}
