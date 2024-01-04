#![allow(unused)] // Ignore warnings about unused code

use std::{io, string}; // Import the 'io' and 'string' modules from the 'std' crate
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // Declare a tuple named 'my_tuple' with elements of different types
    let my_tuple: (u8, String, f64) = (19, "Tushar".to_string(), 50_000.00);

    // Access and print the second element (String) of the tuple
    println!("Name: {}", my_tuple.1);

    // Destructure the tuple into three variables: v1, v2, and v3
    let (v1, v2, v3) = my_tuple;

    // Print the first element (u8) of the destructure tuple
    println!("Age: {}", v1);
}
