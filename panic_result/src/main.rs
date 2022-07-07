use std::convert::Infallible;

fn main() {
    match test() {
        Ok(t) => println!("{}", t),
        Err(_) => println!("parse error"),
    };
    println!("Hello, world!");
}

fn test() -> Result<String, Infallible> {
    "aaaaa".parse::<String>()
}
