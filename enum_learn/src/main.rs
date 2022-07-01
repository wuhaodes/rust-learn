#[derive(Debug)]
enum Week {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    println!("Hello, world!");
    println!("{:?}", Week::Monday);
    println!("{:?}", Week::Tuesday);
    println!("{:?}", Week::Wednesday);
    println!("{:?}", Week::Thursday);
    println!("{:?}", Week::Friday);
    println!("{:?}", Week::Saturday);
    println!("{:?}", Week::Sunday);
    let home = IpAddr::V4(127, 0, 0, 0);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?}", home);
    println!("{:?}", loopback);
    let a: Option<f32> = None;
    let b = a.unwrap_or(3.2) + 3.7;
    println!("b is {}", b);
}
