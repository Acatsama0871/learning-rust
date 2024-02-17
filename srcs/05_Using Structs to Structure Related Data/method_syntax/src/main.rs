
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // those are methods: use self as argument
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
    fn non_zero_width(&self) -> bool {
        return self.width > 0;
    }
    fn can_hold(&self, another_rec: &Rectangle) -> bool {
        return (self.width >= another_rec.width) && (self.height >= another_rec.height);
    }
    
    // associated functions
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rec1 = Rectangle {
        width: 20,
        height: 30,
    };
    let rec1_area = rec1.area();
    println!("Area: {rec1_area}");
    println!("Non-zero width? {}", rec1.non_zero_width());
    
    let rec2 = Rectangle {
        width: 5,
        height: 10
    };
    println!("Can hold? {}", rec1.can_hold(&rec2));
    
    let square1 = Rectangle::square(32);
    println!("Area? {}", square1.area());
}
