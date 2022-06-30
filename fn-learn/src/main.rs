use chrono::prelude::*;
fn main() {
    // let f = 3;
    // let h = 19;
    // let a = get_max(f, h);
    // println!("the previous val is {} , {}", f, h);
    // println!("the max is {}", a);
    // let c = get_max;
    // let d = c(1, 2);
    // println!("the get_max_val is {}", get_max(2, 13));
    // println!("the max val is {}", d);
    // let m = previous();
    // println!("the max val is {}", m.0);
    let now = Local::now();
    let n = 20;
    println!("the fib {} is {}", n, fib(n));
    println!("the fib1 {} is {}", n, fib1(n));
    println!("the start time is {}", now.timestamp_millis());
    println!("the fib {} is {}", n, fib2(n));
    let now = Local::now();
    println!("the end time is {}", now.timestamp_millis());
}

// fn get_max(num1: u32, num2: u32) -> u32 {
//     if num1 > num2 {
//         num1
//     } else {
//         num2
//     }
// }

/**
 * 递归计算斐波那契数列
*/
fn fib(n: u128) -> u128 {
    if n <= 0 {
        panic!("索引必须为正整数");
    } else if n <= 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

/**
 * 循环计算斐波那契数列
*/
fn fib1(n: u128) -> u128 {
    let mut n = n;
    if n <= 0 {
        panic!("索引必须为正整数");
    }
    let mut p1 = 1;
    let mut p2 = 0;
    let mut now = 1;

    while n > 0 {
        now = p1 + p2;
        p1 = p2;
        p2 = now;
        n -= 1;
    }
    now
}

/**
 * 通项公式计算斐波那契数列 开根号导致精度丢失 超过10就不太准了
 */
fn fib2(n: u128) -> u128 {
    if n <= 2 {
        return 1;
    }
    let n: f64 = n as f64;
    let sqrt_5 = 5.0f64.sqrt();
    (1f64 / sqrt_5 * ((1f64 + sqrt_5) / 2f64).powf(n) - ((1f64 - sqrt_5) / 2f64).powf(n)) as u128
}

// fn previous() -> (i32, i32) {
//     (3, 2)
// }
