
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rec1 = Rectangle {
        width: 20,
        height: 32,
    };
    let rec1_area = calculate_area(&rec1);
    println!("Area: {rec1_area}");
}

fn calculate_area(rec: &Rectangle) -> u32 {
    return rec.height * rec.width;
}
