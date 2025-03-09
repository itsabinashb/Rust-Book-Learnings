//use std::vector
fn main() {

    // This is how a new fresh empty vector is created
    // Note: while creating a new vector we need to specify the type because a vector must be of any type
    let v: Vec<i32> = Vec::new();

    // This is how we create a vector with some values
    let v_2 = vec![1,2,3,4];

    // we can add elements in a vector, for that we use push()
    let mut v_3 = vec![1,2,3];
    v_3.push(4);
    v_3.push(5);
    println!("v_3 is: {:?}", v_3);

    // There are two ways to get element from a vector - (i) by indexing, (ii) by calling get() with target index
    /// by indexing -
    let value_by_indexing = v_3[2];
    /// by get() -
    let value_by_get = v_3.get(2); // it returns: Option<> enum
    /// As get() returns Option<> we can assign it in a variable by using match technique
    match value_by_get {
        Option::Some(i) => println!("value_by_get: {}", i),
        Option::None => println!("No value")
    }

    println!("value_by_indexing: {}", value_by_indexing);


    // Iterating through the vector, we will iterate on v_3
    for i in v_3 {
        println!("{}", i);
    }
}
