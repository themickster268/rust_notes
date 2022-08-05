/* enum IpAddrKind {
    V4,
    V6
}

struct IpAddr{
    kind: IpAddrKind,
    address: String
} */

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
   Quit, // Variant with no data associated with it
   Move { x: i32, y: i32 }, // Variant with named fields like a struct
   Write(String), // Variant that includes a String
   ChangColour(i32, i32, i32), // Variant that includes three i32 values
}

impl Message {
    fn call(&self) {
        
    }
}

fn main() {
    /* let four = IpAddrKind::V4;
    let six = IpAddrKind::V6; */

    /* let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    }; */

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}