#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
    // ...
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

fn main() {
    let c1 = Coin::Nickel;
    let c2 = Coin::Quarter(UsState::Alaska);

    println!("This coin has value {}.", value_in_cents(c1)); // This coin has value 5.
    println!("This coin has value {}.", value_in_cents(c2)); // State quarter from Alaska!\n This coin has value 25.
}
