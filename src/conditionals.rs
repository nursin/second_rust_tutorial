// conditionals used to check the condition of something and act on the result

pub fn run() {
    let age: u8 = 21;
    let check_id: bool = false;
    let knows_person_of_age = true;

    //if else
    if age >= 21 && check_id || age >= 21 && knows_person_of_age {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, you have to leave");
    } else {
        println!("Bartender: Ill need to see your ID");
    }

    //shorthand iff as turnary operator
    let is_of_age = if age >= 21 {true} else {false};
    println!("Is of age: {}", is_of_age)
}