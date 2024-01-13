fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if_let();
}

// fn int_expression() {
//     let number = 3;

//     if number {
//         println!("number was three");
//     }
// }

fn many_ifs() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_as_expression() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

fn different_return_types() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}

fn if_let() {
    let some_num = 123;

    // if let, pretty weird, works similar to a match statement
    if let Ok(value) = "123".parse::<i32>() {
        println!("double that and you get {}!", value * 2);
    }

    match some_num {
        123 => println!("double that and you get {}! using match", some_num * 2),
        _ => {}
    }
}
