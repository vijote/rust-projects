enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Dime => 10,
        Coin::Nickel => 5,
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Quarter(state) =>  {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

// Using Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn main_func() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;

    match dice_roll {
        7 => println!("7 is lucky! take this hat"),
        3 => println!("3 is bad!!! gimme a hat"),
        // catch-all arm, with a variable named other
        other => println!("{} seems good! keep playing!", other)
    }

    match dice_roll {
        3 => println!("you got 3!"),
        // If you don't need to use the value of the catch-all arm, _ is used
        _ => println!("catch-all variable is not used here.")
    }

    match dice_roll {
        3 => println!("you got 3!"),
        // Do nothing, ignore completely
        _ => ()
    }
}