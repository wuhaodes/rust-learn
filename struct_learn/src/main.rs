#[derive(Debug)]
struct User {
    name: String,
    age: u16,
    height: f64,
    weight: f64,
}

impl User {
    /**
     * 自我介绍
    */
    fn description(&self) -> String {
        let mut desc = String::from("I'm ");
        desc.push_str(&self.name);
        desc.push_str(";\n");
        desc.push_str("I'm ");
        desc.push_str(&self.age.to_string());
        desc.push_str(" years old;");
        desc
    }
    /**
     * 获取身体质量指数
    */
    fn get_bmi(&self) -> f64{
        self.weight / self.height.powf(2f64)
    }
}

fn main() {
    println!("Hello, world!");

    let user = User {
        name: String::from("张三"),
        age: 32,
        weight: 54.3f64,
        height: 1.72f64,
    };
    let s = &user;
    let desc = user.description();
    let bmi = user.get_bmi();
    println!("{}", desc);
    println!("{}", s.age);
    println!("the user's bmi is: {}", bmi);
    println!("user is {:?}", user);
    println!("user is {:#?}", user);
}
