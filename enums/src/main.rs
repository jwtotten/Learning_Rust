#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("192.168.0.0"),
    };

    println!("Home IP address: {}", home.address);
    println!("Home IP address kind: {:#?}", home.kind);
}
