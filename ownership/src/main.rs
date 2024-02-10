/*
    Ownership enables Rust to make memory safety guarantees without a garbage collector
*/

/*
    The stack and the heap are part of the memory available to use at runtime.
    Its operations are push onto the stack, and popping off the stack.
    The stack orders things as LIFO, and requires its items to have a known, fixed size.
    The heap is less organized, every item requests an amount of space.
    Allocating on the heap consists on finding an empty spot of memory, mark it as
    being in use, and return a pointer; an address to that location.
    As the pointer to the heap is a known and fixed size, it can be stored on the stack.
    When you need to retrieve the data you must follow the pointer 
 */

pub mod slices;

/*
    Ownership rules:
    - Each value in Rust has an owner
    - There can only be one owner at a time
    - When the owner goes out of scope, the value is dropped
*/
fn main() {
    println!("Hello, world!");

    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
}


fn example() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.



fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
