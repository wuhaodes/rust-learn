mod num;
mod test;
use num::real_number;
use test::{hash_map_test, string_test, vec_test};

fn main() {
    vec_test::test();
    string_test::test();
    hash_map_test::test();
    real_number::test_rn_vec();
}
