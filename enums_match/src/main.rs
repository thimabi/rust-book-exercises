#![allow(unused_variables, dead_code)]

fn main() {
    #[derive(Debug, PartialEq)] // so we can inspect the state and do the comparison in a minute
    enum UsState {
        Alabama,
        Alaska,
        None,
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
                if state != UsState::None {
                    println!("State quarter from {:?}!", state);
                } else {
                    println!("Non-state quarter!");
                }
                25
            }
        }
    }

    println!("{:?}", value_in_cents(Coin::Quarter(UsState::Alabama)));
    println!("{:?}", value_in_cents(Coin::Quarter(UsState::None)));

    // Matching with if-let
    let config_max: Option<u8> = Some(3);
    // let config_max: Option<u8> = None;
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        println!("The maximum is not configured.");
    }
}
