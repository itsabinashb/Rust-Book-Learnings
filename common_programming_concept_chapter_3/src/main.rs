fn main() {
    //                                          Tuple

    // NOTE: tuple is a compound data type, it can hold any type of value, but its size is fix i.e after declaration it's size cannot
    // increased & decreased.
    let tup = (100, 45.7, true, '$', "string");
    let a = tup.0;
    let b = tup.1;
    let c = tup.2;
    let d = tup.3;
    let e = tup.4;

    println!("a = {a}, b = {b}, c = {c}, d = {d}, e = {e}");

    //                                           Array

    // NOTE: array is also like a tuple but it contains only same data types, like tuple it also can't grow or shrink in size after declaration, if you
    // want to make a collection which may grow or shrink you should use vector
    let _arr = [1, 2, 3, 4, 5];
    let _arr1 = ["hello", "Rust"];
    let _arr3 = [3; 4]; // this crate an array like this: [3,3,3,3]

    //                                           Function

    let value = not_main(100, 'h');
    println!("value: {value}");

    //                                           If-else

    //NOTE: In rust we don't write conditions in parenthesis [unlike solidity]
    let b = true;
    let num = if b == true { 10 } else { 0 };
    if num < 10 {
        println!("num is less than 10");
    } else if num > 10 {
        println!("num is greater than 10");
    } else if num == 10 {
        println!("num is equal to 10")
    }

    //                                            Loop

    let list = [1,2,3,4,5];

    // NOTE: How it worked? =>take 1st item from 'list' & put it in 'items', then print. Then take 2nd item from 'list', put it in 'items' then
    // print & so on
    for items in list {
        println!("{items}");
    }
}

// NOTE: we must annotate the parameter name with datatype
fn not_main(value: i32, character: char) -> i32 {
    println!("vaule is {value} & character is {character}");
    value
}
