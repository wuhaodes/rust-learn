use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..100);
    let mut lower: u32 = 0;
    let mut upper: u32 = 100;

    loop {
        println!("please enter a number!");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("read line fail");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                if lower < guess {
                    lower = guess;
                }
                println!("the number is between {} ~ {}", lower, upper);
            }
            Ordering::Greater => {
                if upper > guess {
                    upper = guess;
                }
                println!("the number is between {} ~ {}", lower, upper);
            }
            Ordering::Equal => {
                println!("you guess right");
                break;
            }
        }
    }
}
