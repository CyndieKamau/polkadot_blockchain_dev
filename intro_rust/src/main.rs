// This is my intro to Rust practice script
// It's used as my Rust crash course, before building on polkadot blockchain
// Cyndie, 2023.


#![allow(unused)]

use std::io;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
const PI:f32 = 3.14159;  //constant values, written in SCREAMING_CASE

// printing a statement

fn main() {
    println!("Hello world");

    // variables are declared using let
    //variables are written in snake_case
    let x = 15;
    println!("{}", x);

    //mutable variable (read and write to) is denoted by "mut" keyword

    let mut x = 27;
    println!("{}", x);
    x = 6;
    println!("{}", x);


    //basic types and type conversion
    //unsigned integers (u8, u16, u32, u64, u128)
    //signed ints (i8, i16, i32, i64, i128)
    //floats (f32, f64)
    //tuple(a,b,c), array[a,b,c]

    //1. Basic Type conversion
    let a = 14u16;
    let b = 24i8;
    let c = a as i8 + b;
    println!("{}", c);

    println!("The universe follows the {} mechanism.", PI);
    
    //arrays in [type;no] format
    //index is usize (starts at 0)
    let d: [u16;4] = [1,2,3,4];
    println!("{:?}", d); 
    println!("{}", d[1]);

    //array slicing
    let slice = &d[1..3];
    println!("{:?}", slice);
    
    //add function
    println!("20 + 25 = {}", add(20, 25));

    //subtract function
    println!("50 - 13 = {}", subtract(50, 13));

    //swap function return a tuple having 2 values
    let val = swap(10, 15);
    println!("{} {}", val.1, val.1); // returns 10 10 (swap fn swaps indices)
    println!("{} {}", val.0, val.1); // returns 15 10 

    //destructuring the tuple to 2 vars
    let (var1, var2) = swap(val.1, val.0);
    println!("var1 = {}, var2 = {}", var1, var2);

    //mix fn returning tuple
    let ret = mix(44, 5.79, "hey you");
    println!("{} {} {}", ret.0, ret.1, ret.2);

    //destructuring to 3 vars
    let (a, b, c) = mix(ret.0, ret.1, ret.2);
    println!("a = {}, b = {}, c = {}", a, b, c);
    
    //if/else
    let mut no = if_else(20);
    println!("{:?}", no);
    no = if_else(50);
    println!("{:?}", no);

    
    //while loop function
    while_loop(50);


    //for loop
    for x in 0..5 {    //prints from 0 to 4
        println!("{}", x);
    }

    for x in 0..=5 {
        println!("{}", x);   //prints from 0 to 5
    }


    //matching function
    match_no(0);    //prints "found zero"
    match_no(2);    //prints "found one or two"
    match_no(5);    //prints "found from three to nine"
    match_no(67);   //prints "found 67 no between 10 and 100"
    match_no(200);  //prints "not found! Shame "


}

//addition
fn add(x:i32, y:i32) -> i32 {
    return x + y;

}

//subtraction
fn subtract(x:i32, y:i32) -> i32 {
    return x - y;
}


//multiple return values using tuples

//function to swap i32 numbers
fn swap(x:i32, y:i32) -> (i32, i32) {
    return (y, x);

}

//function to return a tuple with int, float and str
fn mix(x:i32, y:f32, z:&str) -> (i32, f32, &str) {
    return (x, y, z);
}


//if/else function

fn if_else(x:i32) {
    if x < 45 {
        println!("less than 45");
    } 
    else if x == 45 {
        println!("is 45 ");
    } 
    else {
        println!("Is greater than 45");
    }

}


// while loop
fn while_loop(x:i32) {
    let mut x = 0;
    while x != 54 {
        x += 1;

    }
    println!("x is {}", x);
}


//matching function

fn match_no(num:i32)  {
    match num {
       0 => {
        println!("Found Zero");
       }

       1 | 2 => {    //matching multiple values
        println!("Found One or Two");
       }

       3..=9 => {   //matching range from 3 to 9
        println!("Found from three to nine")
       }

       matched_no @ 10..=100 => {   //creating variable for a matched number
        println!("Found {} number between 10 and 100", matched_no);
       }

       _ => {   //any other case
        println!("Not found! Shame....")
       }


    }

}