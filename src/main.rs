#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn mian(&self) -> u32 {
        self.height * self.width
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let m = rect1.mian();
    println!("rect1 is {}", m);
}