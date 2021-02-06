// Creating Structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        &self.width * &self.height
    }
    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        &self.width > &rectangle.width && &self.height > &rectangle.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    // Instantiating Structs
    // let rect = Rectangle{width: 10, height: 50};
    let mut rect = Rectangle{width: 10, height: 50};

    // Access Struct elements
    println!("Accessing element -> width: {} height: {}", rect.width, rect.height);

    // Change Struct elements
    rect.width = 15;

    println!("Accessing element post change -> width: {} height: {}", rect.width, rect.height);

    // Tuple Structs
    struct Color(u32, u32, u32);
    let black = Color(0, 0, 0);

    // Access smt in t-structs
    println!("Accessing element t-struct -> first: {} second: {}", black.1, black.2);

    // println!("{:?}", array) -> Debug
    // println!("{}", array) -> 
    
    println!("{:?}", rect);

    let rect1 = Rectangle{width: 1, ..rect};
    println!("{:#?}", rect1);

    fn new_rect(width: u32, height: u32) -> Rectangle {
        Rectangle{
            width,
            height,
        }
    }

    let rect2 = new_rect(5, 32);
    println!("{:#?}", rect2);

    println!("Area: {}", rect.area());
    println!("Does rect fit into rect2? {}", rect.can_hold(&rect2));

    let square = Rectangle::square(10);
    println!("{:#?}", square)
}