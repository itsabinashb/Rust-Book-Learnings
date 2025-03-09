enum Currency {
    Dollar,
    Rupees,
    Dinar
}

fn main() {
    let currency = Currency::Dollar; 
    let _returned_value = simple_match_test(&currency);
    assert_eq!(_returned_value, 84 );

    // matching with Option<t> enum
    let some_ = Option::Some(10);
    let num = match_option(&some_);
    println!("Returned value from match_option(): {num}");

    // Check catch-value
    catch_all(100);
    catch_all_but_do_not_do_anything(100);
    catch_all_and_do_not_bind_the_value(100);

}

fn simple_match_test(currency: &Currency) -> u8 {

    // 'match' allows us to compare a value with a series of pattern & execute code based on which pattern matches.
    // Here, `currency` is value & Currency::Dollar, Currency::Dinar, Currency::Rupees are patterns. As we set the
    // value of `currency` as Dollar so it will match with Currency::Dollar pattern & execute the code connected with
    // the pattern.
    match currency {
        Currency::Dollar => {
            println!("Great! sweet money!");
            84
        }
        Currency::Dinar => {
            println!("Okay");
            24
        }
        Currency::Rupees => {
            println!("Don't want");
            1
        }
    }

}

// Note: the argument type is Option<u32> because the function was called by Option::Some(10), as the Some() holds u32 data type so
// Option has become of u32 type, thats why the argument type is Option<u32> type

fn match_option(_some: &Option<u32>) -> u32 { // here _some == Some(10)
    match _some {
        Option::Some(i) => {
            let num: u32 = _some.unwrap();   // @note we can convert a Option<u32> type to u32 type by using unwrap()
            num + i
        }
        Option::None => {
            println!("As it is None we are returning 0");
            0
        }
    }
}


// Note: 'match' has a feauture called catch-all, like if the value does not match with few pattern then consider the value
// of 'rest of all pattern' & execute the code.

fn catch_all(value: u32) {
    match value {
        10 => println!("It is 10"),
        20 => println!("It is 20"),

        // @note if  the value != 10 & 20 then value will be matched with this pattern, and it *binds with the 'value'*, so we can 
        // use it in code. 
        other => {
            println!("value entered in catch-all field because the value is : {value} & we can also print the 'other', it is {other}");
        }
    }
}

// Note: If we do not want to bind then we can use _ syntax

fn catch_all_and_do_not_bind_the_value(value: u32) {
    match value {
        10 => println!("It is 10"),
        20 => println!("It is 20"),

        // @note if  the value != 10 & 20 then value will be matched with this pattern, But as we using '_' so just the code will execute,
        // we can't bind the 'value' with pattern
        _ => {
            println!("value entered in catch-all field because the value is : {value}");
        }
    }
}

fn catch_all_but_do_not_do_anything(value: u32) {
    match value {
        10 => println!("It is 10"),
        20 => println!("It is 20"),

        // @note if  the value != 10 & 20 then value will be matched with this pattern, But we are not running any code for the value
        _other => (), // @note we could use _ instead of '_other'
    }
}