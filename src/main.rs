use std::io;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;

fn main() {
    // Doubt: we are taking integer but why using String here?
    let mut value = String::new();
    
    println!("Guess a number & put that number:");
    // io is a module, stdin() is a function of that module, this function returns Stdin struct, this struct has a function read_line()
    // which takes user's input
    io::stdin().read_line(&mut value).expect("Not executed correctly");

    // Note: here trim() removes any blank spaces in `value`, the parse() type cast the value in our needed form, we have to mention
    // our needed type with `:`, as we did `: u32` because we needed the string become a u32 type
    let value: u32 = value.trim().parse().expect("Not type casted successfully!");
    // println!("You guessed: {}", value);
    println!("You guessed: {value}");

    // Generating random number
    let secret_num = thread_rng().gen_range(1..=100);
    println!("The random number is: {}", secret_num);

    // Comparing secret number
    match value.cmp(&secret_num) {
        Ordering::Equal => println!("You win! Congrats!"),
        Ordering::Greater => println!("Too large! You loose!"),
        Ordering::Less => println!("Too small! You loose!")

    }

    
}