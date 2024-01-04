// This line prevents warnings about unused variables
#![allow(unused)]

// Import the standard input/output library, similar to #include<stdio.h> in C/C++
use std::io;

// Import the random number generator from the "rand" crate, like importing randint in Python
use rand::Rng;

// Import specific items from the "io" module for a more focused usage
use std::io::{Write, BufReader, BufRead, ErrorKind};

// Import the File type from the "fs" module for file operations
use std::fs::File;

// Import the Ordering enum from the "cmp" module in the standard library
// This enum is commonly used for comparing values and determining their ordering relationship
use std::cmp::Ordering;

// The main function is the entry point of the program
fn main() {
    // Print a message to the console
    println!("What is your name?");
    
    // Declare a mutable variable named "name" to store user input
    let mut name = String::new(); // 'let' is used to define variables and constants
    
    // Declare a constant named "greeting" with a fixed message
    let greeting = "Nice to meet you!";

    // Read a line of input from the user and store it in the 'name' variable
    io::stdin().read_line(&mut name)
        .expect("Didn't receive an input");

    // Print a personalized greeting using the user's name
    println!("Hello, {}! {}", name.trim_end(), greeting);

    // Declare a constant named "TEN_THOUSAND" with a value of 10,000
    const TEN_THOUSAND: u32 = 10_000;

    // Declare a constant named "PI" with an approximate value of 3.14
    const PI: f32 = 3.14;

    // Declare a variable named "age" with the initial value as a string "47"
    let age = "47";

    // Parse the string "47" into an unsigned 32-bit integer and store it in the variable "age"
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned a number");

    // Increment the value of "age" by 1
    age += 1;

    // Print a message indicating the age and a desired first salary
    println!("I'm {} and I want ${} as my first salary", age, TEN_THOUSAND);
}
