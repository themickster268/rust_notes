#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        // functions with '&self' parameters are called methods
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle { // This is not a method, as it doesn't have the &self parameter
        Rectangle {                     // This is called an associated function
            width: size,
            height: size,
        }
    }
}

pub fn run() {
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rec2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rec3 = Rectangle {
        width: 60,
        height: 45,
    };

    let square = Rectangle::square(30);

    println!(
        "The area of the rectangle (rec1) is {} square pixels.",
        rec1.area()
    );

    println!(
        "The area of the square is {} square pixels",
        square.area()
    );

    println!("The rectangle has a nonzero value, it is {}", rec1.width());

    println!("Can rect1 hold rect2? {}", rec1.can_hold(&rec2));
    println!("Can rect1 hold rect3? {}", rec1.can_hold(&rec3));
}
