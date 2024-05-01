use std::{fs::File, io::ErrorKind};

fn main() {
    panic!("crash and burn");
}

fn use_result_instead_of_panic() {
    let some_file_result = File::open("some-file.txt");

    let some_file = match some_file_result {
        Ok(file) => file,
        Err(error) => panic!("An error happened: {:?}", error),
    };
}

fn handle_possible_outcomes() {
    let some_file_result = File::open("some-file.txt");

    let some_file = some_file_result.unwrap_or_else(|err| {
        if err.kind() == ErrorKind::NotFound {
            File::create("some-file.txt").unwrap_or_else(|error| {
              panic!("An error happened: {:?}", error);
            })
        } else {
            panic!("An error happened: {:?}", err);
        }
    });
}

fn using_unwrap() {
    // Unwrap will return the file handle if the result is Ok
    // And in case it failed, will call panic
    let some_file_result = File::open("some-file.txt").unwrap();
}

fn using_expect() {
    // Expect will try to return the file handle if the result is Ok
    // And if it fails it will display the received message
    let some_file_result = File::open("some-file.txt").expect("some-file.txt doesn't exist!");
}