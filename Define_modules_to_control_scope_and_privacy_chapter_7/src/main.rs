fn main() {
    let _num = crate::first_module::add();
    println!("num: {}", _num);

    //let _num_from_sub = crate::second_module::sub();
    let _num_from_sub = second_module::sub(); // @note here we are using relative path, because the module second_module was defined at same level
                                              // of main(). 
    println!("_num_from_sub: {}", _num_from_sub);

    println!("mul: {}", inline_module::mul());
}

// First we created a module, the compiler will look for the code of this module in First_module.rs file because we declared the module here but
// we put the code in First_module.rs file.
mod first_module;

// Secondly we created a module, but for the code of this module the compiler will look for a directory/file name, as there is a directory named
// second_module so compiler will get the code there. 
mod second_module;

mod inline_module {
    // We are making the function public so that we can use it. 
    pub fn mul() -> u32 {
        2*2
    }
}