fn main() {
    my_module::my_function();
}

mod my_module {
    pub fn my_function() {
        println!("Hello from my_module!");
    }
}

// crate can be either binary or library. Library crates are like normal library (in solidity). There can't be more than 1 library crate but a package
// can contain multiple binary crate. 