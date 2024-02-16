fn main() {
    println!("Hello, world!");
    another_hello();
    let x = 32;
    function_with_arguments(x);
    print_labeled_measurements(32, 'm');
    expression_example();
    let five_val = five();
    println!("{five_val}");
}

fn another_hello() {
    println!("Another hello!");
}

fn function_with_arguments(x: i32) {
    println!("The x value is {x}");
}

fn print_labeled_measurements(value: u32, measure: char) {
    print!("The measure is {value}{measure}\n");
}

fn expression_example() {
    let y = {
        let x = 3;
        x + 1  // will return value because it does not have ";"
    };
    
    println!("y value is {y}");
}

fn five() -> i32{
    return 5;
}
