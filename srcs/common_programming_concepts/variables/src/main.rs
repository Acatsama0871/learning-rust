fn main() {
    // assign value to mutable
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
    
    // const
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The constant value is {THREE_HOURS_IN_SECONDS}");
    
    // showing
    let x = 5;
    let x = x + 1;
    println!("The x value is {x}");
    {
        let x = x * 2;
        println!("The x value is {x}");
    };
    println!("The x value is {x}");
}
