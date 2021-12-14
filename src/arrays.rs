//Arrays are fixed list where elements are the same data types

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    numbers[2] = 20;

    println!("{:?}", numbers);

    // Get single val
    println!("Single Value: {}", numbers[0]);

    //get array lenght
    println!("Array length: {}", numbers.len());

    //Arras are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    //get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}