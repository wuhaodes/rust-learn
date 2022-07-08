pub mod lifecycle {
    use std::fmt::Display;
    use rand::Rng;

   & #[derive(Debug)]
    struct LongLifeStr<'a> {
        part: &'a str,
    }

    impl LongLifeStr<'_> {
        fn level(&self)->i32{
            rand::thread_rng().gen_range(1..101)
        }
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
        if str1.len() > str2.len() {
            str1
        } else {
            str2
        }
    }

    fn longest_an<'a, T>(str1: &'a str, str2: &'a str, an: T) -> &'a str where T: Display {
        println!("{}", an);
        if str1.len() > str2.len() {
            str1
        } else {
            str2
        }
    }

    // fn l1<'a>(str1: &'a str, str2: &'a str)-> &'a str {
    //     str1
    // }

    // fn create_str(x: &str, y: &str) -> String {
    //     println!("{}-----{}", x, y);
    //     String::from("create a new str from string")
    // }

    fn first_word(s: &str) -> &str {
        for (i, &item) in s.as_bytes().iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }
        &s[..]
    }

    pub fn test() {
        println!("lifecycle test");

        let r = 231_321;
        {
            let x = 32;
            // r = &x;
            println!("x: {}", x);
        }

        println!("r: {}", r);

        let str1 = String::from("long string is long");
        let res;

        {
            let str2 = String::from("short str");
            res = longest(&str1[..], &str2[..]);
            println!("The longest string is {}", res);
        }
        // res被拿出去使用 str2的生命周期不够长 error
        // println!("The longest string is {}", res);

        println!("str1 is {}", str1);

        let str2 = String::from("ssssssss ssssss");
        let f1 = str2.split(" ").next().expect("can't find space to split");
        let long_str = LongLifeStr { part: f1 };
        println!("{}", f1);
        println!("{:?}", long_str);
        println!("{}", long_str.part);
        let str3 = "it's a dog";
        let first = first_word(&str3);
        println!("{}", first);
        println!("{}", str3);
    }
}
