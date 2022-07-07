pub mod g_type {
    #[derive(Debug)]
    struct Rect<T> {
        x: T,
        y: T,
    }

    impl<T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy> Rect<T> {
        fn area(&self) -> T {
            self.x * self.y
        }
        fn perimeter(&self) -> T {
            self.x + self.x + self.y + self.y
        }
    }

    impl<T: std::cmp::PartialOrd + Copy> Rect<T> {
        fn get_length(&self) -> T {
            if self.x >= self.y {
                self.x
            } else {
                self.y
            }
        }
        fn get_width(&self) -> T {
            if self.x <= self.y {
                self.x
            } else {
                self.y
            }
        }
        fn is_square(&self)->bool{
            self.x == self.y
        }
    }

    /**
     * 获取最大值
     * trait std::cmp::PartialOrd 可以比较
     * trait std::fmt::Debug debug {:?}
     * Clone 可以调用clone方法
     */
    fn get_max<T: std::cmp::PartialOrd + std::fmt::Debug + Clone>(list: &Vec<T>) -> T {
        let list = list.clone();
        let mut iter = list.into_iter();
        let mut max = iter.next().expect("list can't be empty");
        for ele in iter {
            println!("{:?}", ele);
            if max <= ele {
                max = ele
            }
        }
        max
    }

    pub fn test() {
        let list: Vec<i32> = vec![32, 56, 68, 34, 73, 44, 99, 23, 67, 11];
        let a = get_max::<i32>(&list);
        let r1: Rect<f32> = Rect { x: 3f32, y: 3.2 };
        let r2: Rect<u32> = Rect { x: 5, y: 6 };
        println!("{:?},{:?}", a, list);
        println!("{:?}", r1);
        println!("r1's width is {}", r1.get_width());
        println!("r1's length is {}", r1.get_length());
        println!("r1 is square or not: {}", r1.is_square());
        println!("r1's perimeter is {}", r1.perimeter());
        println!("r1's area is {}", r1.area());
        println!("{},{}", r1.x, r1.y);
        println!("{},{}", r2.x, r2.y);
    }
}
