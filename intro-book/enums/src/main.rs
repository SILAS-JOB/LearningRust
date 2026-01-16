<<<<<<< HEAD
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    Iowa,
    Ohio,
    NewYork,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State Quarter from {state:?} !");
            25
        }
    }
}

fn main() {}
=======
enum IpvKnd {
    V4,
    V6
}

let ipv4 = IpvKnd::V4;

fn main() {
}
>>>>>>> c033426 (feat: enums)
