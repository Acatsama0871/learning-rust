fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn main() {
    let string1 = "abchhh";
    {
        let string2 = "xyzm";
        
        let result = longest(string1, string2);
        println!("{}", result);
    }
}
