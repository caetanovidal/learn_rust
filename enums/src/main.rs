fn main() {
    let coin1 = Coin::Penny;
    let coin2 = Coin::Quarter(UsState::Alaska);

    let value = value_in_cents(coin2);
    println!("The value of coin2 is {} cents", value);
}


#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
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
            println!("State quarter from {:?}!", state);
            25
        }
    }
}