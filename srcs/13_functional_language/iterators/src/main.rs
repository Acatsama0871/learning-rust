fn main() {
    let a = vec![1, 2, 3];
    let b = a.iter();
    for val in b {
        println!("Got {}", val);
    }
    
    let a = vec![1, 2, 3];
    let mut b = a.iter();
    assert_eq!(b.next(), Some(&1));
    assert_eq!(b.next(), Some(&2));
    assert_eq!(b.next(), Some(&3));
    
    let b = a.iter();
    let sum_val: i32 = b.sum();
    println!("Sum value {}", sum_val);
    
    // map
    let b = vec![1, 2, 3];
    let a: Vec<i32> = b.iter().map(|x| x + 1).collect();
    let a_iter = a.iter();
    for i in a_iter {
        println!("{}", i);
    }
}
