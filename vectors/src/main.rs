// To stop the compiler from showing warnings, we can use the following command:
#![allow(unused_variables)]

// Lets add some library to our project
use std::io;        // Another way to import everything from the library is to use the wildcard operator * like this use std::*;
use rand::Rng;      // If you add the library rand and gives you a warning, you can add the following line to the Cargo.toml file under the dependencies section: rand = "0.8.4"

// You can add many libraries at once by separating them with a comma like this: use std::io:: {self, Write};
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// Here in this lesson, we are going to learn about vectors in Rust



fn main() {
    
    // vectors are similar to arrays but they can grow or shrink in size
    // to create a vector, you can use the following syntax:
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    numbers.push(6); // Add 6 to the end of the vector
    numbers.pop(); // Remove the last element from the vector
    numbers.insert(2, 10); // Insert 10 at index 2
    numbers.remove(2); // Remove the element at index 2
    println!("Length: {}", numbers.len()); // Print the length of the vector
    println!("Element at index 2: {}", numbers[2]); // Print the element at index 2
}
