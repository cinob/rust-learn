// 枚举变体
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        println!("call")
    }
}

// #[derive(Debug)]
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String
// }

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(four);
    route(six);

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1")
    // };
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("home is {:#?}, loopback is {:#?}", home, loopback);

    let m = Message::Write(String::from("hello"));
    m.call();
    
    // enum Option<T> {
    //     Some(T),
    //     None
    // }
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
}

fn route(ip_type: IpAddrKind) {
    println!("type is {:#?}", ip_type);
}
