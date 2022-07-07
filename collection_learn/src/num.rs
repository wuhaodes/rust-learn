/**
 * 实数
*/
pub mod real_number {
    use std::ops::Add;

    /**
     * 有理数
     */
    #[derive(Debug, Clone)]
    enum RationalNumber {
        Int(i128),
        Float(f64),
    }

    /**
     * 加法
     */
    impl Add<&RationalNumber> for RationalNumber {
        type Output = RationalNumber;

        #[inline]
        fn add(self, o: &RationalNumber) -> RationalNumber {
            let mut f1: f64 = 0f64;
            let mut i1: i128 = 0;

            let v = vec![&self, o];

            for rn in v {
                match rn {
                    RationalNumber::Int(i) => i1 += i,
                    RationalNumber::Float(f) => f1 += f,
                }
            }

            let a = f1 + i1 as f64;

            if a > a as i128 as f64 {
                RationalNumber::Float(a)
            } else {
                RationalNumber::Int(a as i128)
            }
        }
    }

    /**
     * 实数的方法
     */
    impl RationalNumber {}

    /**
     * 实数测试
     */
    pub fn test_rn_vec() {
        let num_vec: Vec<RationalNumber> = vec![
            RationalNumber::Int(32),
            RationalNumber::Int(24),
            RationalNumber::Float(3.268),
        ];

        let mut num_arr = [
            RationalNumber::Int(32),
            RationalNumber::Int(246),
            RationalNumber::Float(3.268),
        ];

        num_arr[0] = RationalNumber::Float(3.26);

        println!("{:?}", num_vec);

        for ele in &mut num_arr {
            match ele {
                RationalNumber::Int(i) => println!("integer is {}", i),
                RationalNumber::Float(f) => println!("float is {}", f),
            }
        }

        for ele in &num_vec {
            match ele {
                RationalNumber::Int(i) => println!("integer is {}", i),
                RationalNumber::Float(f) => println!("float is {}", f),
            }
        }

        let rn1 = RationalNumber::Float(32.56);

        let rn2 = RationalNumber::Int(312);

        let rn3 = rn1.clone() + &rn2;

        let rn4 = RationalNumber::Int(38);

        let rn5 = rn2.clone().add(&rn4);

        let rn6 = RationalNumber::Float(2.44);

        let rn7 = rn6.clone() + &rn3;

        let rn8 = rn1.clone();

        println!("rn1: {:?} + rn2: {:?} = rn3: {:?}", rn1, rn2, rn3);

        println!("rn2: {:?} + rn4: {:?} = rn5: {:?}", rn2, rn4, rn5);

        println!("rn1: {:?} + rn6: {:?} = rn7: {:?}", rn1, rn6, rn7);

        println!("rn8: {:?}", rn8);

        println!("--------------------");
    }
}
