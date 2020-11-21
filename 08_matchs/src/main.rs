#[derive(Debug)] // 这样可以可以立刻看到州的名称
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

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alabama));
    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);
}


fn value_in_cents(coin: Coin) -> u8 {

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
        25
    } else {
        1
    }
    // match coin {
    //     Coin::Penny => {
    //         println!("Lucky penny!");
    //         1
    //     }
    //     Coin::Nickel => 5,
    //     Coin::Dime => 10,
    //     Coin::Quarter(state) => {
    //         println!("State quarter from {:?}!", state);
    //         25
    //     }
    // }
}
