// variables hold primitive data or references to data
// variables are immutable by default
// rust is a block-scoped language

pub fn run() {
    let name = "Bobby";
    let mut age = 34;
    println!("My name is {} and I am {}", name, age);
    age = 38;

    println!("My name is {} and I am {}", name, age);

    // define constant must add type
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // assign multiple variables at once
    let (my_name, my_age) = ("Bobby", 34);
    println!("{} is {}", my_name, my_age);
}