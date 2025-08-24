pub fn array() {
    let arr: [u8; 3] = [1, 2, 3];
    let other_arr: [u8; 5] = [100; 5];

    println!("index: {}, length: {}", arr[0], other_arr.len());

    println!("{:?}", other_arr);
}