use liquid_lang as liquid;

use liquid::InOut;
#[derive(InOut)]
pub struct MyStruct {
    b: bool,
    i: i32,
}

#[liquid::contract(version = "0.1.0")]
mod noop {
    use super::MyStruct;

    #[liquid(storage)]
    struct Noop {}

    #[liquid(methods)]
    impl Noop {
        pub fn new(&mut self) {}

        pub fn noop(&self) -> MyStruct {
            MyStruct { b: true, i: 0 }
        }
    }
}

fn main() {}
