pub mod trait_i {
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

    pub fn test() {
        let a = vec![18, 22, 64, 34, 56];
        let sum = a.summarize();
        let b: Vec<String> = vec![String::from("aaa"), "bbb".to_string(), "ccc".to_string()];

        println!("Vec<i32> is {:?}", a);
        println!("Vec<i32> summary is {}", sum);
        println!("Vec<&str> is {:?}", b);
        println!("trait test");
    }
}
