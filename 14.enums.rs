#![allow(unused)] // Ignore warnings about unused code

use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// Define an enumeration representing the days of the week
enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

// Implement methods for the Day enumeration
impl Day {
    // Check if the day is a weekend
    fn is_weekend(&self) -> bool {
        match self {
            Day::Saturday | Day::Sunday => true,
            _ => false,
        }
    }
}

fn main() {
    // Declare a variable 'today' with the value Day::Sunday
    let today: Day = Day::Sunday;

    // Match on the value of 'today' and print corresponding messages
    match today {
        Day::Monday => println!("Everyone hates Monday"),
        Day::Tuesday => println!("Donot Day"),
        Day::Wednesday => println!("Hump Day"),
        Day::Thursday => println!("Pay Day"),
        Day::Friday => println!("Almost Weekend"),
        Day::Saturday | Day::Sunday => println!("Weekend"),
    }

    // Print whether today is a weekend using the is_weekend method
    println!("Is today a weekend? : {}", today.is_weekend());
}
