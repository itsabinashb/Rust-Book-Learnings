#[derive(Debug)]
struct Rectrangle {
    width: u32,
    height: u32
}

// Methods are defined within the context of struct/enum/trait. To define a method within the context of struct we will have to use `impl` keyword
impl Rectrangle {
    // The first argument of method is always `self` which is the struct itself
    fn calculate_area(&self) -> u32 {
        self.width * self.height
    }


    // We can create a function which does not take the self as argument, it is called associate function
    fn do_something(num: u32) -> u32 {
        num * 100
    }
}

fn main() {
    let rect = Rectrangle {
        width: 100,
        height: 100
    };

    // Passing `rect` as argument to calculate_area() method
    let area = rect.calculate_area();
    println!("area is {}", area);

    let num = Rectrangle::do_something(100);
    println!("Num is :{}", num);
}





struct Rectrangle {
    width: u32,
    height: u32
}

impl Rectrangle {
   
    // We can create a function which does not take the self as argument
    fn do_something(num: u32) -> u32 {
        num * 100
    }
}

fn main() {
    let rect = Rectrangle {
        width: 100,
        height: 100
    };

    let num = Rectrangle::do_something(100);
    println!("Num is :{}", num);
}