mod generic_type;
use generic_type::g_type;

mod trait_test;
use trait_test::trait_i::{self, Summary};

fn main() {
    g_type::test();
    trait_i::test();
    let v1 = vec![32, 54, 65, 43, 23, 34];
    println!("vec<i32>'s summary is {}", v1.summarize());
    println!("vec<i32>'s max is {}", get_largest(&v1));
    println!("{:?}", v1);
}

fn get_largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &ele in list.into_iter() {
        if ele > largest {
            largest = ele
        }
    }
    largest
}
