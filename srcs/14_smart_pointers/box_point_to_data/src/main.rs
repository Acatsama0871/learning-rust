enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    
    // use reference
    let x = 5;
    let y = &x;
    assert_eq!(x, 5);
    assert_eq!(*y, 5);
    
    // use smart pointer
    let x = 5;
    let y = Box::new(x);
    
    assert_eq!(x, 5);
    assert_eq!(*y, 5);
}