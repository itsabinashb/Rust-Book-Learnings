fn main() {
    let x = 10;
    let y = x;
    println!("{x}{y}");

    //                                                  Reference

    //NOTE: I am not giving you the ownership of this value but i am giving you a reference of this variable so that you can you use it. 

    reference();
    mutable_reference();

    //                                                  Slicing

    let s = String::from("hello world");
    let first_half = slice(&s);
    println!("First half is: {first_half}");

    let s2 = String::from("Charles Wang");
    let val = &s2[0..5];


    let mut s3 = String::from("hello world");

    let word = slice(&s3);  // this is immutable reference, as `&mut s3` was not used instead of `&s3`, `&mut s3` makes it mutable referrence

    println!("the first word is: {word}");

    // NOTE: if we comment out the above print & uncomment the last print then it will give error, because slice() was called with immutable 
    // reference, so the return value of slice() will be immutable reference i.e 'word' holds immutable reference. After that we are calling clear()
    // on s3, the clear() needs mutable reference(because it truncates the string) means here s3 is a mutable reference, after that 'word' was used in println. We know that the scope
    // of a reference starts with where it was set & ends with where it was last used, here the immutable reference of s3 sets when slice() was called
    // & ends with println call. So, between this scope the clear() was called which is immutable reference. As per borrowing rule we can't use an
    // immutable reference in scope of a  mutable reference. Thats why it will give compilation error. To fix it we comment out the last print call &
    // bring the print call above clear() call so that clear() come out of the mutable scope.
    s3.clear(); // error!

    // println!("the first word is: {word}");
}

// NOTE: normal reference
fn reference() {
    let x = String::from("reference");

    // NOTE: here x is saying i will not give x1 ownership of my value but i will give it my reference so that it can use my value
    let x1 = &x;
    println!("{x1}");
}

// NOTE: in mutable reference the borrower says I can't give you the ownership but i am giving you the reference & authority to change my value, let me
// be mutable for that.
fn mutable_reference() {

    // x is a mutable variable so that it's value can be updated
    let mut x = String::from("mutable");

    // as x is a mutable variable every reference it will give those will be mutable reference
    let x1 = &mut x;

    // mutable reference was passed to edit_it()
    let y = edit_it(x1);
    println!("{y}");
    println!("{x}");
}

// NOTE: as the argument is String type & it is a mutable reference 
// NOTE: as the 'value' is a reference the return type will be a reference type
fn edit_it(value: &mut String) -> &String{
    value.push_str(" reference");
    value
}


fn slice(s: &String) -> &str{
    let var = s.as_bytes();

    for (i, &item) in var.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}