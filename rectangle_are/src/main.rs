#[derive(Debug)]
struct Rectangle {
   width: u32,
   length: u32,
}

impl Rectangle { 
    fn area(&self) -> u32 {
        self.width * self.length
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The are of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
}

