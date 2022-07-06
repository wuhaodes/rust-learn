/**
 * Vector测试
*/
pub mod vec_test {
    /**
     * 测试
     */
    pub fn test() {
        let mut v: Vec<i32> = Vec::new();
        v.push(32);
        v.push(28);
        println!("{:?}", v);
        v.push(24);
        println!("{:?}", v);
        let v_of = for_of(&v);
        let v_idx = for_each(&v);
        // let v_r = reduce(&mut v);
        // v.push(18);
        println!("{:?}", v_idx);
        println!("{:?}", v_of);
        // println!("{:?}", v_r);
        // println!("{:?}", v)
        println!("--------------------");
    }

    fn for_each(v: &Vec<i32>) -> Vec<u128> {
        let mut idx: Vec<u128> = Vec::new();
        let mut i: u128 = 0;
        for _ in v {
            idx.push(i);
            i += 1;
        }
        idx
    }

    fn for_of(v: &Vec<i32>) -> (Vec<u128>, &Vec<i32>) {
        let mut idx: Vec<u128> = Vec::new();
        let mut i: u128 = 0;
        for _ in v {
            idx.push(i);
            i += 1;
        }
        (idx, v)
    }

    // fn reduce(v: &mut Vec<i32>) -> Vec<u128> {
    //     v.push(3333);
    //     let mut idx: Vec<u128> = Vec::new();
    //     let mut i: u128 = 0;
    //     for _ in v {
    //         idx.push(i);
    //         i += 1;
    //     }
    //     idx
    // }
}

/**
 * String测试
*/
pub mod string_test {
    pub fn test() {
        let mut s = String::new();
        s.push_str("Hello Everyone!");
        println!("{}", s);
        s.push_str(" sssss");
        let data = "Hello Rust!";
        let s1 = data.to_string();
        println!("{}", s1);
        let s2 = s + &s1;
        println!("{}", s2);
        let mut s3 = String::from("It's");
        let s4 = String::from("a");
        let s5 = String::from("dog");
        let s6 = format!("{} {} {}", s3, s4, s5);
        println!("{}", s3);
        println!("{}", s6);
        s3.push(' ');
        s3.push('a');

        for ele in s3.chars() {
            println!("{}", ele)
        }

        println!("{}", s3);

        println!("--------------------");
    }
}

/**
 * HashMap测试
*/
pub mod hash_map_test {
    use std::collections::HashMap;
    pub fn test() {
        let mut scores = HashMap::new();
        scores.insert(String::from("test"), 10f64);
        scores.insert(String::from("test1"), 10.2f64);
        println!("{:?}", scores.get(&String::from("test")));
    }
}
