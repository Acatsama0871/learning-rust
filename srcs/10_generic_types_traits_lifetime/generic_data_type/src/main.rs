use std::cmp::PartialOrd;

fn largest_integer(input_array: &[i32]) -> &i32 {
    let mut largest_number = &input_array[0];
    
    for i in input_array {
        if i > largest_number {
            largest_number = i;
        }
    }
    
    return largest_number;
}

fn largest_generic<T: PartialOrd>(input: &[T]) -> &T {
    let mut largest_element = &input[0];
    
    for i in input {
        if i > largest_element {
            largest_element = i;
        }
    }
    
    return largest_element;
}

struct Point<T, U> {
    x: T,
    y: U
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let largest_number = largest_integer(&numbers);
    println!("{largest_number}");
    
    let chars = vec!['a', 'b', 'c'];
    let largest_char = largest_generic(&chars);
    println!("{largest_char}");
    
    let float_int_point = Point {x: 1, y: 2.0};
}
