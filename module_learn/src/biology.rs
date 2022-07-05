pub mod animal {
    #[derive(Debug)]
    pub struct Cat {
        pub name: String,
        age: u32,
        pub weight: u32,
    }
    impl Cat {
        pub fn run(&self) {}
        fn self_introduction(&self) ->&Cat{
            println!("{}", self.age);
            self
        }
    }
    impl Cat {
        pub fn new(name: &str, age: u32, weight: u32) -> Cat {
            let cat = Cat {
                name: String::from(name),
                age,
                weight,
            };
            cat.self_introduction();
            cat
        }
    }
}
pub mod plant {
    # [derive(Debug)]
    pub struct Watermelon{
        pub name: String,
        pub weight: f32,
    }
    impl Watermelon {
        pub fn new(name: &str, weight: f32) -> Watermelon {
            Watermelon{
                name: String::from(name),
                weight
            }
        }
    }
    impl Watermelon {
        pub fn description(&self){
            println!("{:#?}", self);
        }
    }
}
