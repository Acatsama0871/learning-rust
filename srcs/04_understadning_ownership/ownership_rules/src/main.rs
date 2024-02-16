fn main() {
    {
        let s = "Hello";
        // when s goes out of the scope, it will be cleaned
    }
    
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
    
    let x = 5;
    let y = x;
    
    let s1 = String::from("hello");  // s1 is no longer valid, its content has been moved to s2
    let s2 = s1;  // instead of data, the pointer is copied here
    
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    
    function_and_ownership();
    return_gives_ownership();
}

fn function_and_ownership() {
    let s = String::from("hello");
    let x: i32 = 77;
    
    take_ownership(s);
    make_copy(x);
    println!("{x}");
}

fn take_ownership(s: String) {
    println!("{}", s);  // the ownership has been took over here
}

fn make_copy(x: i32) {
    println!("{}", x);  // x is passed by copying
}

fn return_gives_ownership() {
    let s = String::from("hello");
    let s1 = takes_and_give_it_bask(s);
    let s2 = give_ownership();
    println!("{}, {}", s1, s2);
}

fn give_ownership() -> String {
    let x: String = String::from("world");
    return x;
}

fn takes_and_give_it_bask(s: String) -> String {
    s
}
