#![allow(unused)] // Ignore warnings about unused code

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // Unsigned integers: u8, u16, u32, u64, u128, usize
    // Signed integers: i8, i16, i32, i64, isize
    // Floating-point numbers: f32, f64

    // Print the maximum value for various numeric types
    println!("Max size of u32: {}", u32::MAX);
    println!("Max size of u64: {}", u64::MAX);
    println!("Max size of usize: {}", usize::MAX);
    println!("Max size of u128: {}", u128::MAX);
    println!("Max size of f32: {}", f32::MAX);
    println!("Max size of f64: {}", f64::MAX);

    let is_true = true;
    let my_grade = 'A'; // Single quote for character
    let my_grades = "AA"; // Double quotes for string
}
