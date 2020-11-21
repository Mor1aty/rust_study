// enum IpAddrType {
//     V4,
//     V6,
// }
//
// struct IpAddr {
//     kind: IpAddrType,
//     address: String,
// }

// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

fn main() {
    // let four = IpAddrType::V4;
    // let six = IpAddrType::V6;
    // route(six);
    //
    // let home = IpAddr {
    //     kind: IpAddrType::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let home = IpAddr::V4(127, 0, 0, 1);
    let m = Message::Write(String::from("hello"));
    m.call()
}

// fn route(ip_type: IpAddrType) {}
