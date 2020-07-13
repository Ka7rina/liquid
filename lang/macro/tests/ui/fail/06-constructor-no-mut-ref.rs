use liquid_lang as liquid;

#[liquid::contract(version = "0.1.0")]
mod noop {
    #[liquid(storage)]
    struct Noop {}

    impl Noop {
        #[liquid(constructor)]
        fn init(&self) {}

        #[liquid(external)]
        fn noop(&self) {}
    }
}

fn main() {}
