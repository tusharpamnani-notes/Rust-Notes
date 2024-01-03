#![allow(unused)] // Ignore warnings about unused code

use std::i32::MAX; // Import the MAX constant from the i32 module
use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // Define the variable 'age' with the value 18
    let age = 18;

    // Use a match statement to evaluate different cases based on the value of 'age'
    match age {
        // If 'age' is in the range 1 to 18 (inclusive), print a birthday message
        1..=18 => println!("Happy Birthday"),
        // If 'age' is 21 or 50, print a different message
        21 | 50 => println!("Not a happy Birthday"),
        // If 'age' is in the range 65 to the maximum value of i32, print an anniversary message
        65..=MAX => println!("Happy Anniversary"),
        // If 'age' doesn't match any of the above cases, print a default message
        _ => println!("Not a Number"),
    };

    // Define the variable 'my_age' with the value 18
    let my_age = 18;
    // Define the variable 'voting_age' with the value 18
    let voting_age = 18;

    // Use the match statement to compare 'my_age' with 'voting_age' using the cmp method
    match my_age.cmp(&voting_age) {
        // If 'my_age' is less than 'voting_age', print a message indicating the inability to vote
        Ordering::Less => println!("You can't vote"),
        // If 'my_age' is equal to 'voting_age', print a message indicating the ability to vote
        Ordering::Equal => println!("You can vote"),
        // If 'my_age' is greater than 'voting_age', print a message indicating the ability to vote and more
        Ordering::Greater => println!("You can also vote"),
    };
}
