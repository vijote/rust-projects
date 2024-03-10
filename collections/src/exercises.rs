use core::panic;
use std::collections::HashMap;

pub fn all_exercises() {
    let numbers = [4,7,3,9,3];
    median_and_mode(numbers);
    string_to_pig_latin(String::from("hola soy tilin"))
}

fn median_and_mode(numbers: [u32; 5]) {
    let mut vec = Vec::from(numbers);
    vec.sort();

    let middle = vec.len().div_ceil(2) - 1;
    let mut number_appearences: HashMap<u32, u32> = HashMap::new();
    let mut highest_appearence: u32 = 0;
    let mut mode: u32 = vec[0];

    for number in &vec {
        let number_value = *number;
        let count = number_appearences.entry(number_value).or_insert(0);
        let count_value = *count;
        *count += 1;

        if count_value > highest_appearence {
            highest_appearence = count_value;
            mode = number_value;
        }
    }
    
    println!("middle: {}, mode: {}", &vec[middle], mode);
}

fn string_to_pig_latin(str: String) {
    let words = str.split_whitespace();
    let vowels = "aeiou";

    for (_size, word) in words.enumerate() {
        println!("{}", word);
        
        for letter in word.chars() {
            if !char::is_alphabetic(letter) {
                panic!("only vowels allowed");
            }

            if vowels.contains(letter) {
                
            }
        }
    }
}