#![allow(unused)] // Ignore warnings about unused code

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // Declare an array named 'arr' containing the values 1 to 9
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    // Initialize a mutable variable 'arr_indx' to keep track of the array index
    let mut arr_indx = 0;

    // Start an infinite loop
    loop {
        // Check if the current array element is even; if so, skip to the next iteration
        if arr[arr_indx] % 2 == 0 {
            arr_indx += 1;
            continue;
        }

        // Check if the current array element is equal to 9; if so, break the loop
        if arr[arr_indx] == 9 {
            break;
        }

        // Print the value of the current array element
        println!("Val: {}", arr[arr_indx]);

        // Increment the array index for the next iteration
        arr_indx += 1;
    }
}
