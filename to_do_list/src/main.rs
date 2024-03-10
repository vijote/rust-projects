use std::{fmt, io};

enum MenuOptions {
    AddToDo = 1,
    CompleteToDo = 2,
    Quit = 3
}

impl fmt::Display for MenuOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MenuOptions::AddToDo => write!(f, "1"),
            MenuOptions::CompleteToDo => write!(f, "2"),
            MenuOptions::Quit => write!(f, "3"),
        }
    }
}

fn main() {
    loop {
        println!("Select operation:");
        println!("{} - Add new ToDo", MenuOptions::AddToDo);
        println!("{} - Complete ToDo", MenuOptions::CompleteToDo);
        println!("{} - Quit", MenuOptions::Quit);

        let user_input = read_input();

        if user_input.parse() {
            break;
        }
    }
}

fn read_input() -> String {
    let mut temporal_read = String::new();

    let result = io::stdin().read_line(&mut temporal_read);

    if let Err(error) = result {
        println!("There was an error with your input: {}", error);
    }

    temporal_read.trim().to_string()
}

fn select_option(raw_option: &String) {
    let parse_result = raw_option.parse::<MenuOptions>();

    if let Err(error) = parse_result {
        println!("There was an error with your input: {}", error);
        panic!();
    }

    match parse_result.unwrap() {
        MenuOptions::AddToDo => {
            println!("adding to do!");
        },
        MenuOptions::CompleteToDo => {
            println!("completing to do!");
        },
        MenuOptions::Quit => {
            println!("quiting!");
        }
    }
}
