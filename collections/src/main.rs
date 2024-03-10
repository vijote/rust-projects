mod strings;
mod hash_maps;
mod exercises;

enum DifferentTypes {
    Int(i32),
    Float(f64),
    Text(String)
}

fn main() {
    // This type annotation is necessary because
    // No value is provided to the new function
    let _v: Vec<i32> = Vec::new();

    // Here the type annotation is not necessary
    // It can infer the type based on the provided values
    let _v_2 = vec![1,2,3];

    // Here the type is not specified either
    // but based on the elements it gets pushed
    // the compiler can infer the type
    let mut v_3 = Vec::new();
    v_3.push(1);
    v_3.push(2);

    // Reading from a vector
    let v_4 = vec![1,2,3];

    let third = &v_4[2];
    println!("the third element is: {}", third);

    let third = v_4.get(2);

    match third {
        None => println!("There's no third element!"),
        Some(third) => println!("The third element is {third}")
    }

    // iterating over a vector
    // using an immutable reference
    let v_iter = vec![1,2,3,4];
    for i in &v_iter {
        println!("{i}");
    }

    let mut v_mut = vec![1,2,3,4];

    // using a mutable reference
    for i in &mut v_mut {
        // to be able to mutate the value this mutable reference points to
        // first it's necessary to dereference to get the value of 'i'
        *i += 50;
    }

    // Vectors allow only one type by default, but using them
    // with an enum extend their possibilities, allowing for
    // diferente types to be stored
    let _vec_mult = vec![
        DifferentTypes::Int(1),
        DifferentTypes::Float(5.2),
        DifferentTypes::Text(String::from("hello"))
    ];

    // When the vector gets dropped its elements are also dropped
    {
        let _vect = vec![1,2,3];
    }

    exercises::all_exercises();
}
