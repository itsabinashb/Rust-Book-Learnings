// NOTE: In Rust crate root is a top level module of the crate. So if we declare a function here then we can say this function is in top level module. So,
// if we declare any other module later in this crate that module will be child module of the top level module. 

// Lets define a function
fn function_of_top_module(){}

mod front_of_the_house {
    pub mod hosting {
        pub fn add_to_waitlist(){}
    }

    // Now call the function of top module, as front_of_the_house is child module so we can use 'super' keyword to declare the path for that top level 
    // module's function
    fn call_parent_module() {
        super::function_of_top_module();
        // The `super` keyword works exactly like how we use in solidity, to call parent contract(in rust module)'s function
    }
}

pub fn eat_at_restaurant(){
    // This is absolute path, because the path starts from crate root
    crate::front_of_the_house::hosting::add_to_waitlist(); 
    // This is relative path, because the path starts from the current module
    front_of_the_house::hosting::add_to_waitlist();

    // Here both path will work properly because -
    // (i) to be called function & caller function is present in same crate -  that is why absolute path will work
    // (ii)as the module of to be called function is defined at the same level of caller function - that is why the relative path will work
}


mod another_module {
    pub struct public_struct {
        pub name: String 
    }

    // declaring a method within the context of this struct
    impl public_struct {
        pub fn set_struct(&self) -> &String {
            &self.name
        }
    }
}

fn call_another_module() {
    let pub_struct = another_module::public_struct{
        name: String::from("struct from another module")
    };
    let _string = pub_struct.set_struct();
    println!("{}", _string);
}