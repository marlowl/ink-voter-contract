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
        pub fn increment_vote(&mut self) {
            self.vote_count += 1;
        }

        #[ink(message)]
        pub fn decrease_vote(&mut self) {
            self.vote_count = self.vote_count - 1
        }

        #[ink(message)]
        pub fn get_votes(&mut self) -> i32 {
            self.vote_count
        }

        #[ink(message)]
        pub fn increment_my_vote_count(&mut self, by: i32) {
            let caller = self.env().caller();
            let id = self.get_my_vote_count();
            self.increment_vote();
            self.id.insert(caller, &(id + by));
        }

        #[ink(message)]
        pub fn get_my_vote_count(&mut self) -> i32 {
            self.id.get(&self.env().caller()).unwrap_or_default()
        }

        #[ink(message)]
        pub fn remove_my_vote_count(&mut self) {
            self.id.remove(&self.env().caller());
            self.decrease_vote()
        }
    }
}