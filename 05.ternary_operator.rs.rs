#![allow(unused)] // Ignore warnings about unused code

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // Declare a mutable variable 'my_age' with the value 19
    let mut my_age = 19;

    // Use an if-else expression to determine if the person can vote
    let can_vote = if my_age >= 18 {
        true
    } else {
        false
    };

    // Print whether the person can vote
    println!("Can Vote: {}", can_vote);
}
