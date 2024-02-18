
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsStates)
}

#[derive(Debug)]
enum UsStates {
    Alabama,
    Alaska,
}

fn value_in_coin(coin: Coin) -> u8 {
    match coin {
        Coin::Dime => {
            return 10;
        },
        Coin::Nickel => {
            return 5;
        },
        Coin::Penny => {
            return 1;
        }
        Coin::Quarter(state) => {
            println!("{:?}", state);
            return 25;
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}


fn main() {
    let coin = Coin::Quarter(UsStates::Alabama);
    value_in_coin(coin);
    
    let some_val = Some(32);
    match plus_one(some_val) {
        Some(val) => println!("{}", val),
        None => println!("The value is none")
    };
    
    let dice_roll = 8;
    match dice_roll {
        9 => println!("roll is 9"),
        _ => println!("roll is not 9")  // this will catch all
    }
}
