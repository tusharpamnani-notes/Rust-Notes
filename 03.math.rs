#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


fn main(){
    let num_1: f32 = 1.11111111111111111;
    let num_2: f64 = 1.11111111111111111;

    // Print numbers with different precision
    println!("f32: {}", num_1 + 0.1111111111111111); // 6-digit precision
    println!("f64: {}", num_2 + 0.1111111111111111); // 15-digit precision

    let num_3: u32 = 5;
    let num_4: u32 = 4;

    // Perform basic arithmetic operations
    println!("5 + 4 = {}", num_3 + num_4);
    println!("5 - 4 = {}", num_3 - num_4);
    println!("5 * 4 = {}", num_3 * num_4);
    println!("5 / 4 = {}", num_3 / num_4);
    println!("5 % 4 = {}", num_3 % num_4);

    // Generate and print a random number between 1 and 100
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random number: {}", random_num);
}
