#![allow(unused)] // Ignore warnings about unused code

use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // Declare an unsigned 8-bit integer 'int_u8' with the value 5
    let int_u8: u8 = 5;

    // Declare another unsigned 8-bit integer 'int2_u8' with the value 4
    let int2_u8: u8 = 4;

    // Convert 'int_u8' and 'int2_u8' to unsigned 32-bit integers and add them
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);

    // Print the result of the addition
    println!("{}", int3_u32);
}
