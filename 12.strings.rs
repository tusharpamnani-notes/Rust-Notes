#![allow(unused)] // Ignore warnings about unused code

use std::{io, string}; // Import the 'io' and 'string' modules from the 'std' crate
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // Declare a mutable String named 'st1'
    let mut st1 = String::new();

    // Append a character 'A' to the string
    st1.push('A');

    // Append a string " Word" to the existing string
    st1.push_str(" Word");

    // Print the contents of the string
    println!("{}", st1);

    // Iterate over words in the string and print each word
    for word in st1.split_whitespace() {
        println!("{}", word);
    }

    // Replace occurrences of "A" with "Another" in the string and create a new String 'st2'
    let st2 = st1.replace("A", "Another");

    // Print the modified string
    println!("{}", st2);

    // Create a String 'st3' and convert its characters to a sorted and deduplicated Vec<char>
    let st3 = String::from("t u s h a r p a m n a n i");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}", char);
    }

    // Create a string slice 'st4' and convert it to a String 'st5'
    let st4: &str = "Random String";
    let mut st5 = st4.to_string();
    println!("{}", st5);

    // Convert 'st5' to a byte array 'byte_arr1'
    let byte_arr1 = st5.as_bytes();

    // Create a string slice 'st6' from 'st5' and print its length
    let st6 = &st5[0..6];
    println!("String Length : {}", st6.len());

    // Clear the contents of 'st5'
    st5.clear();

    // Concatenate the contents of 'st6' and a reference to 'st7' to create a new string 'st8'.
    let st6 = String::from("Just some");
    let st7 = String::from(" words");
    let st8 = st6 + &st7;
    // In this line, the + operator is used for string concatenation in Rust. The + operator takes ownership of the left operand (st6 in this case) and borrows the right operand (&st7). This operation creates a new string (st8) that contains the combined contents of st6 and st7. The + operator is implemented for the String type in Rust, allowing for convenient string concatenation.

    // Iterate over the bytes of 'st8' and print each ASCII value
    for byte in st8.bytes() {
        println!("{}", byte);
    }
}
