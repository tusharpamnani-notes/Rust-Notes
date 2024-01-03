#![allow(unused)] // Ignore warnings about unused code

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // Define the variable 'age' with the value 8
    let age = 8;

    // Check if age is between 1 and 18 (inclusive)
    if (age >= 1) && (age <= 18) {
        println!("You are a child");
    }
    // Check if age is less than or equal to 65 (after checking the first condition)
    else if (age <= 65) {
        println!("You are an adult");
    }
    // If none of the above conditions are met, assume the person is a senior citizen
    else {
        println!("You are a senior citizen");
    }
}
