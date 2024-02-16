fn main() {
    let s = String::from("Hello World!");
    let hello = &s[..5];  // no begining to indicate it starts from 0
    let world = &s[5..];  // no ending to indicates it ends till string end
    let hello_world = &s[..];
    
    println!("{hello}{world}");
    println!("{hello_world}");
    
    let first_word = first_word(&s);
    println!("{first_word}");
    
    // string literals are string slice
    let s = "Hello, world!";
    
    // array slice
    let a = [1, 2, 3, 4, 5, 6];
    let slice = &a[2..4];
    assert_eq!(slice, &[3, 4]);
}

fn first_word(s: &str) -> &str{ // type from string slice
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    
    return &s[..];
}