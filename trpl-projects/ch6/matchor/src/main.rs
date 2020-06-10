enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("lucky penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(us_state) => {
            println!("[?]={:?}", us_state);
            25
        }
    }
}

fn main() {
    let c = Coin::Penny;
    let val = value_in_cents(c);
    println!("[?]={}", val);
    let c2 = Coin::Quarter(UsState::Alabama);
    let val = value_in_cents(c2);
    println!("[?]={}", val);
    let unit = println!("lol");
    println!("[?]={:?}", unit);
}

