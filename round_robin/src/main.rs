use std::io;
fn main() {
    let servers: [u8; 3] = [1, 2, 3];

    let mut user_input = String::new();

    let mut current_server = 0usize;

    loop {
        println!("Please type a new request");
        let request = io::stdin().read_line(&mut user_input);

        match request {
            Ok(input) => input,
            Err(_) => continue,
        };

        println!("Request sent to server: {}", servers[current_server]);

        current_server = if current_server < servers.len() - 1 {
            current_server + 1
        } else {
            0
        };
    }
}
