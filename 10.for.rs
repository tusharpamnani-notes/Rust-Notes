#![allow(unused)] // Ignore warnings about unused code

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // Declare an array named 'arr' containing the values 1 to 9
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    // Initialize a mutable variable 'arr_index' to keep track of the array index
    let mut arr_index = 0;

    // Use a for loop with the iter() method to iterate through the elements of the array
    for val in arr.iter() {
        // Print the value of the current array element
        println!("Val: {}", val);
    }
}
