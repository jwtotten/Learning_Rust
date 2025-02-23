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

    println!("________________________________________________________");
    println!("Example of Option enum");
    println!("________________________________________________________");

    // You dont need to include the Option enum in the prelude
    // because it is used so often and it is included in the prelude
    // by default.

    let some_number= Some(5);
    let some_character = Some('a');

    let absent_number:Option<i32> = None;
    
}
