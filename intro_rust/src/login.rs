#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod account_manager {
    #[ink(storage)]
    pub struct AccountManager {
        accounts: ink_storage::collections::HashMap<AccountId, Account>,
    }

    #[derive(Debug, Clone, PartialEq, Eq, scale::Encode, scale::Decode)]
    pub struct Account {
        username: String,
        password: String,
    }

    impl Account {
        pub fn new(username: String, password: String) -> Self {
            Self { username, password }
        }
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
                if account.username == username && account.password == password {
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
}






use ink_lang::contract;
use ink_prelude::string::String;

#[contract]
mod account_manager {
    use ink_storage::{
        collections::HashMap as StorageHashMap,
        lazy::Lazy,
    };

    #[ink(storage)]
    pub struct AccountManager {
        /// Stores the next unique account ID to be used.
        next_account_id: Lazy<u32>,
        /// Maps the username to the account ID.
        username_to_account_id: StorageHashMap<String, u32>,
        /// Maps the account ID to the account information.
        accounts: StorageHashMap<u32, Account>,
    }

    /// Represents a user account.
    #[derive(Debug)]
    pub struct Account {
        address: AccountId,
        username: String,
        password: String,
    }

    impl AccountManager {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                next_account_id: Lazy::new(1),
                username_to_account_id: StorageHashMap::new(),
                accounts: StorageHashMap::new(),
            }
        }

        #[ink(message)]
        pub fn create_account(&mut self, username: String, password: String) -> u32 {
            // Ensure that the username is not already taken.
            assert!(!self.username_to_account_id.contains_key(&username), "Username already taken.");

            // Create a new account.
            let account_id = *self.next_account_id;
            let account = Account {
                address: self.env().caller(),
                username: username.clone(),
                password,
            };
            self.accounts.insert(account_id, account);
            self.username_to_account_id.insert(username, account_id);

            // Increment the next account ID.
            *self.next_account_id += 1;

            account_id
        }

        #[ink(message)]
        pub fn get_account(&self, account_id: u32) -> Option<Account> {
            self.accounts.get(&account_id).cloned()
        }

        #[ink(message)]
        pub fn get_account_by_username(&self, username: String) -> Option<Account> {
            self.username_to_account_id.get(&username).and_then(|account_id| {
                self.accounts.get(account_id).cloned()
            })
        }
    }
}
