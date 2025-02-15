#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

fn main() {

    let home = IpAddrKind::V4(String::from("192.168.0.0"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    println!("Home IP address: {:#?}", home);
    println!("Loopback IP address: {:#?}", loopback);
}
