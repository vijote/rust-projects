fn something() {
    // initializes an empty string
    let _empty = String::new();

    // creates a string with 'initial' as starting value
    let _s = "initial".to_string();

    // string contents can be added using the + operator to concatenate
    let mut test_str = String::from("hello ");
    test_str = test_str + "world";

    // or can also be formatted using format! macro, similar to println!
    format!("hello {}", "world");

    // a string can be updated using push str, and a string slice
    test_str.push_str("another");

    // the push method can also be used, with a character
    test_str.push('l');

    let s1 = String::from("hello");
    let s2 = String::from("world");

    // string concatenation requires an owned String on the left
    // that's why it's necessary to use a moved value
    // instead of two references
    let s3 = s1 + &s2;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // for more complicated string combining the format! macro can be used
    // plus it uses references so the strings are not moved
    let s4 = format!("{s1}-{s2}-{s3}");
}