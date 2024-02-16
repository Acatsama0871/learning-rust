fn main() {
    // immutable reference
    let s1: String = String::from("Hello, world1");
    let s1_size = calculate_size(&s1);
    println!("String: {s1} with size: {s1_size}");
    
    // mutable reference
    let mut s1: String = String::from("Hello");
    finish_hello_world(&mut s1);
    println!("{s1}");
    multiple_immutable_ref_to_same_object();
}

fn calculate_size(s: &String) -> usize {
    return s.len();
}

fn finish_hello_world(s: &mut String) {
    s.push_str(", world!");
}

fn multiple_immutable_ref_to_same_object() {
    let mut s = String::from("I am some value");
    let r1 = &s;
    let r2 = &s;
    println!("r1: {r1}, r2: {r2}");
    
    let r3 = &mut s;
    println!("r3: {r3}");
}
