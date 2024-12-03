fn main() {
    let home = IpAddrKind::V4(Ipv4Addr{});
    let loopback = IpAddrKind::V6(Ipv6Addr{});
    route(home);
    route(loopback);
    //
    
    let m = Message::Write(String::from("hello"));
    m.call();
    //

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six is: {:?}\nnone is {:?}",
        six, none  
    );
    //
    
    let dice_roll: u8 = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        // coult also use: _ => reroll(), // _ is if we don't need the variable
        // could also do: _ => (), // _ we don't need the value; () the unit value (the code won't do anything)
    }
    //

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}

struct Ipv4Addr {}
struct Ipv6Addr {}

enum IpAddrKind {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
fn route(ip_kind: IpAddrKind) -> IpAddrKind {
    ip_kind
}
//

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Calling...");
    }
}
//

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u8{
        match self {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarted from {state:?}!");
                25
            },
        }
    }
}
//

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
//

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}