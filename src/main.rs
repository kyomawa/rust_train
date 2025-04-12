#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        self::Rectangle { height, width }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_contain(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 4,
    };

    let rect2 = Rectangle::new(12, 6);

    println!("{:#?}", rect);
    println!("{:#?}", rect.area());
    println!("{:#?}", rect2);
    println!("{:#?}", rect2.can_contain(&rect));
}
