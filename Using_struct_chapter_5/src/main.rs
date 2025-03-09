// Declaring a normal struct

struct User {
    name: String,
    age: u32,
    job: String,
    passion: String,
}

fn main() {
    
    // How to use a struct? / How to create struct instance?
    let user = User {
        name: String::from("Abinash Burman"),
        age: 28,
        job: String::from("Smart contract auditor"),
        passion: String::from("Workout")
    };

    // How to read a struct? => dot notation
    println!("name: {}, age: {}, job: {}, passion: {}", user.name, user.age, user.job, user.passion);

    // Mutable struct so that it's field can be updated
    let mut user2 = User {
        name: String::from("Abinash Burman"),
        age: 28,
        job: String::from("Smart contract auditor"),
        passion: String::from("Workout")
    };

    user2.name = String::from("Who is this guy?");
    user2.job = String::from("What does he do for living?");
    user2.passion = String::from("What is his passion?");

    println!("{}",user2.name);

    // Passing struct's field in parameter so that the function can set the values in the struct
    let returned_struct = return_struct(true, 100);
    println!("value is: {}", returned_struct.value);


    // Creating an instance of struct with another instance using Update syntax (..<instance_name>)
    // instances must be of same struct
    let user3 = User {
        name: String::from("No name"),
        ..user
    };

    println!("{} & {}", user3.name, user3.age);

    // passing struct as argument, just like normal argument, it is so easy

    let rectrangle = Rectrangle {
        height: 10,
        width: 10
    };
    let _area = area(&rectrangle);
    println!("Area is {}", _area);
    println!("rectrangle is: {:#?}", rectrangle);

}


struct Data {
    true_or_false: bool,
    value: u32
}

// Passing values as parameter so that its value can be set & returning the struct
fn return_struct(true_or_false: bool, value: u32) -> Data{
    let data = Data {
        true_or_false: true_or_false,
        value: value
    };

    data
}

#[derive(Debug)]
struct Rectrangle {
    height: u32,
    width: u32
}

fn area(rectrangle: &Rectrangle) -> u32 {
    rectrangle.width * rectrangle.height
}
