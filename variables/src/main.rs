// Constant
// It's gonna be evaluated in compile time
// so a few operations can be done at this point
// Type annotation is required
const CONSTANT_VALUE: u32 = 1 + 1 + 1;

fn main() {
    // Mutability
    // Useful for value reassignment
    let mut x = 5;

    // Shadowing
    // Useful for type overriding
    let x = x + 1;

    {
        let x = x * 2;

        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
