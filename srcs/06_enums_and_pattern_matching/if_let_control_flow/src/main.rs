fn main() {
    let config_max: Option<i32> = Some(32);
    match config_max {
        Some(value) => println!("The value is {}", value),
        None => ()
    }
    
    if let Some(max) = config_max {
        println!("The max value is {}", max);
    } else {
        print!("no value");
    }
}
