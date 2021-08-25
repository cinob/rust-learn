enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

fn main() {
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three"),
        // 为了满足match表达式穷尽性的需求, 不得已使用_ => ()
        _ => ()
    }

    // if let 更加简短
    if let Some(3) = some_u8_value {
        println!("three");
    }

    let mut count = 0;
    let coin = Coin::Dime;

    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:#?}", state),
    //     _ => count += 1
    // }

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:#?}", state);
    } else {
        count += 1;
    }
    println!("Other count {:#?}", count);
}
