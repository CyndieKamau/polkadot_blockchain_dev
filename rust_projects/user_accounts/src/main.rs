// This rust code is a simple implementation of managing user accounts and balances

#![allow(unused)]

use std::io;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;


struct UserAccounts {
    user_name: String,
    user_bal: f64,

}



//function to get user input
fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

//function for creating an account
fn create_account(accounts: &mut HashMap<String, UserAccounts>, user_name: String, user_bal: f64) {
    // Create a new Account struct with the given name and balance
    let account = UserAccounts { user_name: user_name.clone(), user_bal };

    // Add the account to the dictionary using the name as the key
    accounts.insert(user_name.clone(), account);
}


fn main() {
    //creating an account
    // 1. Creating a new empty dir to store the accounts
    let mut accounts: HashMap<String, UserAccounts> = HashMap::new();

    //2. Add some initial accounts to the dir.
    
    create_account(&mut accounts, String::from("Cyndie"), 2000.0);
    create_account(&mut accounts, String::from("Jordan"), 5000.0);


    //3. Print the initial state of the dictionary
    println!("Initial accounts:");
    for (name, account) in &accounts {
        println!("{}: ${}", name, account.user_bal);
    }

    //4. Prompt the user to create a new account
    println!("Create a new account:");
    println!("Enter name:");
    let name = get_input();
    println!("Enter initial balance:");
    let balance: f64 = get_input().parse().expect("Invalid balance");

    //5. Call the create_account function to add the new account to the dictionary
    create_account(&mut accounts, name, balance);

    //6. Print the updated state of the dictionary
    println!("Updated accounts:");
    for (name, account) in &accounts {
        println!("{}: ${}", name, account.user_bal);
    }


}




