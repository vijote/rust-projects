pub mod match_flow;

enum IpAddrKind {
    V4,
    V6
}

struct IpAddrStruct {
    address: String,
    kind: IpAddrKind
}

enum IpAddrEnum {
    V4(String),
    V6(String)
}

enum IpAddrDT {
    V4(u8, u8, u8, u8),
    V6(String)
}

// This could be implemented with different structs, or be grouped together under a single struct 
enum Message {
    Quit, // no data associated
    Move {x: i32, y: i32}, // with named fields
    Write(String), // includes a string
    ChangeColor(i32, i32, i32) // includes three i32
}

impl Message {
    fn call(&self) {
        println!("Calling!");
    }
}

fn main() {
    // Using variants namespaced with colon
    let ipv4 = IpAddrKind::V4;
    let ipv6 = IpAddrKind::V6;

    // Combining structs and enums
    let home = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddrStruct {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home_enum = IpAddrEnum::V4(String::from("127.0.0.1"));

    let loopback_enum = IpAddrEnum::V6(String::from("::1"));

    let home = IpAddrDT::V4(127, 0, 0, 1);

    let loopback = IpAddrDT::V6(String::from("::1"));
    
    let m = Message::Write(String::from("Hello"));

    m.call();
}

// A function can use any kind of ip address
fn route(ip_kind: IpAddrKind) {}