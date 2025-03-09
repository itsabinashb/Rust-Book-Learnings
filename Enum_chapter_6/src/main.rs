enum Shapes {
    Circle,
    Square,
    Triangle
}


// Just like Struct enum let us create implementations
impl Shapes {
    // we can create method 
    fn enum_method(&self) {
        
    }

    // we can create associate functions
    fn enum_associate_function(num: u32) -> u32 {
        num
    }
}

fn main() {
    create_enum_instance();
    call_enum_variant();

    let shapes = Shapes::Circle;
    shapes.enum_method();
    let num = Shapes::enum_associate_function(100);
    println!("Number is: {num}");

    // we can use Option<T> enum in this way
    let _option = Option::Some(100);
    let _b = _option.is_some();
    println!("_b is {_b}");
    // also in this way
    let option_ = Some(200);
    let _c = option_.is_some();
    println!("_c is {_c}");

}

fn create_enum_instance() {
    let _circle = Shapes::Circle;
    let _square = Shapes::Square;
    let _triangle = Shapes::Triangle;
}


enum IpAddr {
    V4(u32,u32,u32,u32),
    V6(String)
}

fn call_enum_variant() {
    let _v4 = IpAddr::V4(10,10,10,10);
    let _v6 = IpAddr::V6(String::from("::1"));

}
