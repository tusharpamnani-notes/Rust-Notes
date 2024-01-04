#![allow(unused)] // Ignore warnings about unused code

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // Declare an array named 'arr' containing the values 1 to 9
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    // Initialize a mutable variable 'arr_ind' to keep track of the array index
    let mut arr_ind = 0;

    // Start a while loop that continues as long as 'arr_ind' is less than the length of the array
    while arr_ind < arr.len() {
        // Print the value of the current array element
        println!("Arr: {}", arr[arr_ind]);

        // Increment the array index for the next iteration
        arr_ind += 1;
    }
}
