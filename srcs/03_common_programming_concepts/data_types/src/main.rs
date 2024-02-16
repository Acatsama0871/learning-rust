fn main() {
    // tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;  // destrubting
    println!("The value of z is {z}.");
    let temp = tup.0;
    println!("Access tuple with tup.0: {temp}");
    
    // array type: all elements need to have the same type, saved on stack, not heap.
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 3] = [0, 1, 2];  // array type annotation
    let c = [3; 5];
    let first = c[0];
    let second: i32 = c[4];
    println!("The first element {first}");
    println!("The second element {second}");
}
