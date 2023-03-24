#![allow(unused)]

use std::io;
use rand::Rng;
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


}
