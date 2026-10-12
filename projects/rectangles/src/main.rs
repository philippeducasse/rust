#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    let rectangle2 = Rectangle {
        width: 1,
        height: 100,
    };

    let square = Rectangle::square(5);
    println!("The rectangle is {rectangle:#?}");
    println!("The square is {square:#?}");

    println!("The are of the rectangle 1 is {}", rectangle.area());
    println!(
        "Rectangle 1 can hold rectangle 2?:  {}",
        rectangle.can_hold(&rectangle2)
    );
}
