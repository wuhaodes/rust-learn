fn main() {
    owner();
    let a: [u128; 5] = [12, 34, 56, 55, 22];
    let b = a;
    println!("a is {}", a[0]);
    println!("b is {}", b[0]);

    let s1 = String::from("Hello");
    let s2 = get_total_string(s1);
    println!("s2 is ({}, {})", s2.0, s2.1);
}

// move
fn get_total_string(mut s: String) -> (String, usize) {
    s.push_str(" World!");
    let len = s.len();
    (s, len)
}

// move、copy、clone
fn owner() {
    let s = 3;

    let b = {
        let mut s = String::from("hello");

        println!("inner s is {}", s);

        s.push_str(" world");

        println!("inner s is {}", s);
        // clone方法
        let s3 = s.clone();
        // 移动
        let s2 = s;

        println!("inner s2 is {}", s2);

        println!("inner s3 is {}", s3);

        3
    };

    // 赋值操作: 栈上的内容会被拷贝,堆上的内容需要调用clone才会被拷贝
    // copy注解
    // Rust 不允许自身或其任何部分实现了 Drop trait 的类型使用 Copy trait
    let mut x = 5;
    let y = x;
    x = x * 256;

    println!("x is {}", x);
    println!("y is {}", y);
    println!("s is {} and block result is {}", s, b);
}
