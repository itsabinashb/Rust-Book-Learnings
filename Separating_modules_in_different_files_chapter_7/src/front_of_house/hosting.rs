pub fn add_to_waitlist(){}

// So why we create a directory called front_of_house for this file??
// Because hosting module is child of front_of_house module, we could directly set the hosting.rs in /src directory, but in that case the compiler would expect
// the module of this function to be in crate root, not as children of front_of_house module, as hosting module is a children of front_of_house so we have to
// create a directory with the same name of the parent module.
/*
Here is error we get if we put the hosting.rs in ./src/ directory:

error[E0583]: file not found for module `hosting`                                                                                                                                                             
 --> src/front_of_house.rs:1:1
  |
1 | pub mod hosting;
  | ^^^^^^^^^^^^^^^^
  |
  = help: to create the module `hosting`, create file "src/front_of_house/hosting.rs" or "src/front_of_house/hosting/mod.rs"
  = note: if there is a `mod hosting` elsewhere in the crate already, import it with `use crate::...` instead
*/