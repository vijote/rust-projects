fn main() {
    // Two data types: scalar and compound
    // Scalars: integer, float, bool and chars

    // SCALARS: single value

    // integers can be from 8bit up to 128, and can be signed or unsigned
    let num: u8 = 1;

    let signed: i16 = 32;

    // The default float value is 64bit
    let float: f64 = 2.1; // double precision
    let single: f32 = 3.0; // single precision

    let boolean: bool = true;

    // chars are unicode values so they can contain letters, emojis, chinese and japananese chars, etc;
    let character: char = 'z';
    let another: char = '👌';

    // COMPOUND: they can group multiple values into one

    // tuple
    let tup: (u32, char, f32) = (255, 'a', 1.125);

    // destructuring
    let (x, y, z) = tup;

    // accessing with period
    let first = tup.0;

    // Empty tuple: unit
    let un = ();

    // array always has a fixed length and type
    let array = [1, 2, 3, 4, 5];

    let array_with_same_elem = [3; 5];

    // access with index
    let first = array[0];
}
