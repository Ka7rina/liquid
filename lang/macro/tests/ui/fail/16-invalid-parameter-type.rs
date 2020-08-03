use liquid_lang as liquid;

#[liquid::contract(version = "0.1.0")]
mod noop {
    #[liquid(storage)]
    struct Noop {}

    #[liquid(methods)]
    impl Noop {
        pub fn constructor(&mut self) {}

        pub fn noop(&self, value: f32) {}
    }
}

fn main() {}
