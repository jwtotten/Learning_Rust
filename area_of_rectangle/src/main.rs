#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other_rectangle: Rectangle) -> bool {
        self.width > other_rectangle.width && self.height > other_rectangle.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 10,
        height: 60,
    };


    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Checking if rect 2 can fit within rect 1: {}", rect1.can_hold(rect2));
    // At this point, rect1 has an immutable borrow of rect2. 
    // This means that rect2 is dropped as it goes out of scope after 
    // rect1 finished the can_hold method call.
    println!("Accessing rect2's area: {}", rect2.area());
    // The above line of code should throw a compliation error.
    println!("Checking if rect 3 can fit within rect 2: {}", rect1.can_hold(rect3));
}