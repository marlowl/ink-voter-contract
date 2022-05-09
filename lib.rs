#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod voter {
    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Voter {
        vote_count: i32,
        id: ink_storage::Mapping<AccountId, i32>,
    }

    impl Voter {
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.vote_count = init_value;
                let caller = Self::env().caller();
                contract.id.insert(&caller, &0);
            })
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.vote_count = Default::default();
            })
        }

        #[ink(message)]
        pub fn increment_my_vote(&mut self) {
            let caller = self.env().caller();
            let id = self.get_my_vote();
            let increment = id + 1;
            self.increment_vote();
            self.id.insert(caller, &(increment));
        }

        #[ink(message)]
        pub fn decrement_my_vote(&mut self) {
            let caller = self.env().caller();
            let id = self.get_my_vote();
            let decrement = id - 1;
            self.decrement_vote();
            self.id.insert(caller, &(decrement));
        }

        #[ink(message)]
        pub fn get_my_vote(&self) -> i32 {
            self.id.get(&self.env().caller()).unwrap_or_default()
        }

        #[ink(message)]
        pub fn increment_vote(&mut self) {
            self.vote_count = self.vote_count + 1;
        }

        #[ink(message)]
        pub fn decrement_vote(&mut self) {
            self.vote_count = self.vote_count - 1
        }

        #[ink(message)]
        pub fn get_votes(&self) -> i32 {
            self.vote_count
        }
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;

        use ink_lang as ink;

        #[ink::test]
        fn test_default() {
            let contract = Voter::default();
            assert_eq!(contract.get_my_vote(), 0);
        }

        #[ink::test]
        fn test_increment_my_vote() {
            let mut contract = Voter::default();
            assert_eq!(contract.get_my_vote(), 0);
            contract.increment_my_vote();
            assert_eq!(contract.get_my_vote(), 1);
            contract.increment_my_vote();
            assert_eq!(contract.get_my_vote(), 2);
        }

        #[ink::test]
        fn test_decrement_my_vote() {
            let mut contract = Voter::default();
            assert_eq!(contract.get_my_vote(), 0);
            contract.increment_my_vote();
            assert_eq!(contract.get_my_vote(), 1);
            contract.decrement_my_vote();
            assert_eq!(contract.get_my_vote(), 0);
        }

        #[ink::test]
        fn test_get_votes() {
            let mut contract = Voter::default();
            contract.increment_my_vote();
            assert_eq!(contract.get_votes(), 1);
            contract.decrement_my_vote();
            assert_eq!(contract.get_votes(), 0);
        }
    }
}
