pub mod trait_i {
    use rand::Rng;
    use std::fmt::{Debug, Display};

    pub trait Summary<T> {
        fn summarize(&self) -> T;
    }

    impl<T: std::ops::AddAssign + Copy> Summary<T> for Vec<T> {
        fn summarize(&self) -> T {
            let mut it = self.clone().into_iter();
            let mut s = it.next().expect("vector can't be empty");
            for ele in it {
                s += ele;
            }
            s
        }
    }

    fn some_fn<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
        println!("{}, {:?}", t, u);
        rand::thread_rng().gen_range(1..101)
    }

    fn some_fn1<T, U>(t: T, u: U) -> i32
    where
        T: Display + Clone,
        U: Debug + Clone,
    {
        println!("{}, {:?}", t, u);
        rand::thread_rng().gen_range(1..101)
    }

    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for item in list {
            if largest < *item {
                largest = *item
            }
        }
        largest
    }

    pub fn notify<T: Display + Clone>(item: &T) -> T {
        println!("{}", item);
        item.clone()
    }

    pub fn notify1(item: &(impl Display + Clone)) -> impl Display + Clone {
        println!("{}", item);
        item.clone()
    }

    pub fn test() {
        let a = vec![18, 22, 64, 34, 56];
        let sum = a.summarize();
        let b: Vec<String> = vec![String::from("aaa"), "bbb".to_string(), "ccc".to_string()];

        println!("Vec<i32> is {:?}", a);
        println!("Vec<i32> summary is {}", sum);
        println!("Vec<&str> is {:?}", b);
        println!("trait test");
        let t = some_fn(1, 2);
        println!("{}", t);
        let q = some_fn1(32, 65.5);
        println!("{}", q);
        let big = largest(&a);
        println!("a's max value is {}", big);
        let str1 = String::from("it's a dog");
        let d = notify(&str1);
        let s = notify1(&d);
        println!("{}", s);
        println!("{}", d);
        println!("{}", str1);
    }
}
