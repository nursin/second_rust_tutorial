//vectors are resizable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    numbers[2] = 20;

    //add on to vectors'
    numbers.push(5);
    numbers.push(6);

    //pop off
    numbers.pop();

    println!("{:?}", numbers);

    // Get single val
    println!("Single Value: {}", numbers[0]);

    //get Vector lenght
    println!("Vector length: {}", numbers.len());

    //Arras are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    //get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    //loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    //loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}