fn main() {
    // if condition
    let number = 3;
    if number < 5 {
        println!("The condition is true");
    } else {
        println!("The condition is false");
    }
    
    if number != 0 {
        print!("The number is non zero\n");
    }
    
    let condition = true;
    let number = if condition {5} else {6};
    println!("{number}");
    
    let number = 9;
    if number % 2 == 0 {
        println!("The number is dividable by 2");
    } else if number % 3 == 0{
        println!("The nunmber is dividable by 3");
    } else {
        println!("Not dividable by 2 and 3");
    }
    
    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter;  // value after break will be returned
        }
    };
    println!("The result is {result}");
    loop_label_example();
    liftoff_countdown();
    for_loop_example();
    for_loop_range_example();
}

fn loop_label_example () {
    let mut counter = 0;
    
    'counting_up: loop {
        println!("Counter: {counter}");
        let mut remaining = 10;
        loop {
            println!("remaining: {remaining}");
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        counter += 1;
    }
}

fn liftoff_countdown () {
    println!("countdown starts...");
    let mut countdown = 10;
    
    while countdown != 0 {
        println!("Countdown {countdown}");
        countdown -= 1;
    }
}

fn for_loop_example () {
    println!("For loop array example");
    let a = [1, 2, 3, 4, 5, 6];
    
    for i in a {
        println!("i : {i}");
    }
}

fn for_loop_range_example () {
    println!("for loop range example");
    for number in (1..4).rev() {
        println!("Count down: {number}");
    }
}
