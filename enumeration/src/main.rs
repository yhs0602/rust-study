#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method content
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}


fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let some_number = Some(5);
    let some_string = Some("A string");
    let absent_number : Option<i32> = None;

    let five_plus_one = plus_one(some_number);
    let none_plus_one = plus_one(absent_number);

    println!("five plus one is {:?}, none plus one is {:?}", five_plus_one, none_plus_one);

    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("Three");
    } else {
        println!("Not three");
    }

    println!("Hello, world!");
}

fn route(ip_type: IpAddrKind) {
    print!("{:?}", ip_type);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}
