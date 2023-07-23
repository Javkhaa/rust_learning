struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }

    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn new_square(width: u32) -> Rectangle {
        Rectangle { width: width, height: width }
    }

}

fn main() {
    let mut rect = Rectangle { width: 10, height: 5 };
    println!("old area: {}", rect.area());
    rect.inc_width(5);
    println!("new area: {}", rect.area());
    let mut line = String::new();
    let width: i32 = std::io::stdin().read_line(&mut line).parse().unwrap();
    let my_rectangle = Rectangle::new_square {width: width, height: width};
    println!("new area: {}", my_rectangle.area());
}