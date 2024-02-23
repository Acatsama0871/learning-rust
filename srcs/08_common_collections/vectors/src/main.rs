

fn main() {
    // create a new vector
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];  // create with marco
    let mut v: Vec<i32> = Vec::new();
    v.push(3);
    v.push(2);
    v.push(1);
    
    // get elements in the vector
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    let third = v.get(2);
    match third {
        Some(val) => println!("The value of the item is {}", val),
        None => println!("No value")
    };
    
    // iterating over the values
    let v = vec![2, 3, 4];
    for i in &v {
        println!("The value is {}", i)
    }
    let mut v_mut = vec![2, 3, 4];
    for i in &mut v_mut {
        *i *= 50;
        println!("The value is {}", i);
    }
    
    // using enum for multiple types
    enum Cell {
        Int(i32),
        Float(f32),
        Text(String)
    }
    let row = vec![
        Cell::Int(32),
        Cell::Float(5.6),
        Cell::Text(String::from("hello world"))
    ];
    for i in &row {
        match  i {
            Cell::Text(val) => println!("{}", val),
            Cell::Int(val) => println!("{}", val),
            Cell::Float(val) => print!("{}\n", val),
        };
    }
}
