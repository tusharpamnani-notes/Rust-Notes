use std::io;
use std::io::Write;

// Function to print a greeting with the provided name
fn say_hello(a: &String) {
    println!("Hello {}", a);
}

// Function to print the sum of two integers
fn get_sum(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}

// Function to calculate and return the sum of two integers
fn get_sum_2(x: i32, y: i32) -> i32 {
    x + y
}

// Function to calculate and return a tuple with the sum of x+1 and x+2
fn sum_3(x: i32) -> (i32, i32) {
    (x + 1, x + 2)
}

// Function to calculate and return the sum of elements in a list
fn sum_of_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list.iter() {
        sum += val;
    }
    sum
}

fn main() {
    // Print the prompt before taking input for the name
    print!("Enter your name: ");
    io::stdout().flush().unwrap();

    // Read input into the 'name' variable
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");

    // Call the say_hello function with the entered name
    say_hello(&name);

    // Call the get_sum function with two integer arguments
    get_sum(5, 8);

    // Call the get_sum_2 function and print the result
    println!("{}", get_sum_2(5, 4));

    // Call the sum_3 function and print the tuple values
    let (val_1, val_2) = sum_3(2);
    println!("{} {}", val_1, val_2);

    // Create a vector of integers and call the sum_of_list function
    let num_list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("Sum of the list : {}", sum_of_list(&num_list));
}
