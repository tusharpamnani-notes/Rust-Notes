#![allow(unused)] // Ignore warnings about unused code

use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // Create an empty vector of type i32
    let vect: Vec<i32> = Vec::new();

    // Create a vector with initial values
    let mut vec2 = vec![1, 2, 3, 4, 5];

    // Add a new element to the vector
    vec2.push(6);

    // Print the contents of the vector using a for loop
    println!("Vector:");
    for i in &vec2 {
        println!("{}", i);
    }

    // Access the first element of the vector
    println!("1st : {}", vec2[0]);

    // Access the second element using indexing
    let second: &i32 = &vec2[1];
    println!("Second : {}", second);

    // Access the second element using the get method and match
    match vec2.get(1) {
        Some(second) => println!("Second : {}", second),
        None => println!("Match not found"),
    };

    // Iterate over mutable references and multiply each element by 2
    for i in &mut vec2 {
        *i *= 2;
    }

    // Print the modified vector
    for i in &vec2 {
        println!("{}", i);
    }

    // Print the length of the vector
    println!("Vec length : {}", vec2.len());

    // Remove the last element from the vector and print it
    println!("Pop : {:?}", vec2.pop());
}
