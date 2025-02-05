// To stop the compiler from showing warnings, we can use the following command:
#![allow(unused_variables)]

// Lets add some library to our project
use std::io;        // Another way to import everything from the library is to use the wildcard operator * like this use std::*;
use rand::Rng;      // If you add the library rand and gives you a warning, you can add the following line to the Cargo.toml file under the dependencies section: rand = "0.8.4"

// You can add many libraries at once by separating them with a comma like this: use std::io:: {self, Write};
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// In this lesson we will learn about tuples in Rust
fn main() {
    
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    // Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same.
    // Tuples are useful when you want to return multiple values from a function without using a struct.
    // Tuples are also useful when you want to pass a number of values to a function, but don’t want to create a struct.
    // Tuples are created by enclosing the values in parentheses and separating them with commas.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
}
