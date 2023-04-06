#![cfg_attr(not(feature = "std"), no_std)]
#![no_std]

use ink_lang as ink;
use ink_storage::collections::HashMap;
use ink_storage::{
    collections::HashMap as StorageHashMap,
    traits::{PackedLayout, SpreadLayout},
};
use ink_prelude::{string::String, vec::Vec};
use core::hash::Hash;
use ink_env::{
    hash::{Sha2x256, Sha3x256},
    DefaultSrmlTypes,
};
use scale::{Decode, Encode};
use sha3::{Digest, Keccak256};
/// The storage key type used by the ERC20 contract.
pub type Key = [u8; 32];


#[ink::contract]
mod account_manager {
    use ink_prelude::vec::Vec;
    use scale::Hash;

    #[derive(scale::Encode, scale::Decode, SpreadLayout, PackedLayout)]
    #[cfg_attr(feature = "std", derive(Debug))]
    pub struct Account {
        pub username: String,
        pub password_hash: Hash,
    }

    impl Account {
        pub fn new(username: String, password: String) -> Self {
            let mut password_bytes = password.as_bytes().to_vec();
            password_bytes.push(0); // append null terminator
            let password_hash = ink_env::hash::keccak256(password_bytes.as_slice());
            Self { username, password_hash }
        }

        pub fn verify_password(&self, password: &str) -> bool {
            let mut password_bytes = password.as_bytes().to_vec();
            password_bytes.push(0); // append null terminator
            let password_hash = ink_env::hash::keccak256(password_bytes.as_slice());
            self.password_hash == password_hash
        }
    }

    #[ink(storage)]
    pub struct AccountManager {
        accounts: StorageHashMap<AccountId, Account>,
    }

    #[ink(event)]
    pub struct AccountCreated {
        #[ink(topic)]
        account_id: AccountId,
        username: String,
    }

    #[ink(event)]
    pub struct AccountLoggedIn {
        #[ink(topic)]
        account_id: AccountId,
        username: String,
    }

    impl AccountManager {
        #[ink(constructor)]
        pub fn new(&mut self) {
            self.accounts = Default::default();
        }

        #[ink(message)]
        pub fn create_account(&mut self, username: String, password: String) -> bool {
            let caller = self.env().caller();
            if self.accounts.contains_key(&caller) {
                return false;
            }
            let account = Account::new(username.clone(), password.clone());
            self.accounts.insert(caller, account);
            self.env().emit_event(AccountCreated {
                account_id: caller,
                username,
            });
            true
        }

        #[ink(message)]
        pub fn login(&self, username: String, password: String) -> bool {
            let caller = self.env().caller();
            if let Some(account) = self.accounts.get(&caller) {
                if account.username == username && account.verify_password(&password) {
                    self.env().emit_event(AccountLoggedIn {
                        account_id: caller,
                        username: username.clone(),
                    });
                    return true;
                }
            }
            false
        }

        #[ink(message)]
        pub fn get_username(&self) -> Option<String> {
            let caller = self.env().caller();
            if let Some(account) = self.accounts.get(&caller) {
                return Some(account.username.clone());
            }
            None
        }
    }