// Adding this line makes Rust use its default functionality to debug
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Methods impl block
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rectangle: &Rectangle,) -> bool {
        self.width > rectangle.width && self.height > rectangle.height
    }
}

// Associated functions impl block
impl Rectangle {
    // because it doesn't use self
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let sq = Rectangle::square(30);

    // Using {:?} tells the macro to use Debug format
    // Plain {} uses the Display trait
    // which structs don't implement by default
    // With more longer structs the format {:#?} is also an option
    println!("rect1 is {:?}", rect1);

    using_dbg();

    println!("using method: {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_structs(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

fn using_dbg() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
