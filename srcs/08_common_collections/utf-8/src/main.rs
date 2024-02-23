fn main() {
    // create a new string
    let mut s = String::new();
    let data = "I am a data";
    let s = data.to_string();
    
    // update a string
    let mut s = String::from("Hello, ");
    s.push_str("world!");
    println!("{}", s);
    
    // concat
    let s1 = String::from("1 + ");
    let s2 = String::from("1");
    let s3 = s1 + &s2;  // s1 will be take ownership
    println!("{}", s3);
    
    // iterate over string
    for c in "Зд".chars() {
        println!("{c}");
    }    
}
