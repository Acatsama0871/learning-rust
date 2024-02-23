use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Hello", 1);
    scores.insert("world", 2);
    
    let query_word = "Hello";
    let the_score = scores.get(&query_word).copied().unwrap_or(0);  // the copied function converts the Option<&i32> -> Option<i32>
    println!("{}", the_score);
    
    // iterate over hash map
    for (key, value) in &scores {
        println!("{key}-{value}");
    }
    
    // update values
    let mut scores = HashMap::new();
    scores.insert(String::from("hello"), 1);
    scores.insert(String::from("world"), 2);
    scores.insert(String::from("hello"), 3);
    
    for (key, value) in &scores {
        println!("{key} - {value}");
    }
    
    // if not present then insert
    scores.entry(String::from("!")).or_insert(20);
    
    // update value based on old key
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

}
