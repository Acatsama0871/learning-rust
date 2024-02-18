enum IpAddKind {
    v4(String), // enum can take values to init
    v6(String)
}

enum IpAddKindWithType {
    v4(u8, u8, u8, u8),
    v6(String)
}

enum Message {
    Quit,
    Move {x: i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32, i32)
}

impl Message { // implement the methon for enum
    fn call(&self) {
        
    }
}

fn main() {
    let ip4 = IpAddKind::v4;
    let ip6 = IpAddKind::v6;
    let home = IpAddKind::v4(String::from("127.0.0.1"));
    let loopback = IpAddKind::v6(String::from("I am some ip."));
    let home = IpAddKindWithType::v4(128, 0, 0, 1);
    let loopback = IpAddKindWithType::v6(String::from("I am some ip"));
    let m = Message::Quit;
    m.call();
    
    // option enum
    let some_number = Some(5);
    let some_cahr = Some('a');
    let absent_number: Option<i32> = None;
    let x: i8 = 5;
    let y:Option<i8> = Some(4);
    // println!("{}", x + y);  this will create an error
}

fn route(ip_type: IpAddKind) {  // the function takes an enum as input
    
}
