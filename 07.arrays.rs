#![allow(unused)] // Ignore warnings about unused code

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // Declare an array named 'arr_1' containing the values 1 to 9
    let arr_1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    // Print the first element of the array
    println!("1st: {}", arr_1[0]);

    // Print the length of the array
    println!("Length: {}", arr_1.len());
}
