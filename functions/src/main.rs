fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn expressions() {
    // "6" is an expression, which evaluates to 6
    let x = 6;

    // another expression, which evaluates to 3
    let another = 1 + 2;

    // a block expression, the ending line doesn't have an ending semicolon
    let yet_another = {
        let something = 3;
        something + 2
    };

    five();

    // if you add a semicolon to the end of an expression you turn it into a statement
}

// function with return type declared
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}