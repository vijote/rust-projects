fn main() {
    let tup: (u8, u8, u8) = (1, 2, 3);

    let mut count = 0u32;

    loop {
        count += 1;        

        println!("current item is: {}", tup.0);

        if count >= 10 {
            break;
        }
    }
}
