mod biology;
use biology::{animal, plant};

fn main() {
    let cat2 = animal::Cat::new("张三", 32, 32);
    cat2.run();
    println!("{:#?}", cat2);
    plant::Watermelon::new("best",3.2).description();
}
