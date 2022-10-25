#[ink::contract]
mod contract {
    #[ink(storage)]
    pub struct Contract {}

    impl Contract {
        #[ink(constructor, selector = 0)]
        pub fn constructor() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn message(&self) {}
    }
}

use contract::Contract;

fn main() {
    assert!(!<Contract as ::ink::traits::DispatchableConstructorInfo<0>>::PAYABLE);
}
