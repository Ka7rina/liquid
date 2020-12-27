#![cfg_attr(not(feature = "std"), no_std)]

use liquid::storage;
use liquid_lang as liquid;

#[liquid::contract]
mod sum_2 {
    use super::*;

    #[liquid(storage)]
    struct Sum2 {
        values: storage::IterableMapping<String, u32>,
    }

    #[liquid(methods)]
    impl Sum2 {
        pub fn new(&mut self) {
            self.values.initialize();
        }

        pub fn insert(&mut self, key: String, val: u32) {
            self.values.insert(key, val);
        }

        pub fn sum(&self) -> u32 {
            let mut ret = 0u32;
            for (_, v) in self.values.iter() {
                ret += v;
            }
            ret
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn it_works() {
            let mut contract = Sum2::new();
            for i in 0..10 {
                contract.insert(i.to_string(), i);
            }
            assert_eq!(contract.sum(), 45);
        }
    }
}
