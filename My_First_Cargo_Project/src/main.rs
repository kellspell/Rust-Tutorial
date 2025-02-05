// To stop the compiler from showing warnings, we can use the following command:
#![allow(unused_variables)]

// Lets add some library to our project
use std::io;        // Another way to import everything from the library is to use the wildcard operator * like this use std::*;
use rand::Rng;      // If you add the library rand and gives you a warning, you can add the following line to the Cargo.toml file under the dependencies section: rand = "0.8.4"

// You can add many libraries at once by separating them with a comma like this: use std::io:: {self, Write};
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;


fn main() {
    println!("What is your name?");  // Now lets create a variable to store the name of the user name and print it

    // By default, variables in Rust are immutable, so we need to add the keyword mut to make it mutable
    let mut name = String::new();    // The ::new() is an associated function of the String type. Associated functions are implemented on types, rather than on instances of types.
    let greeting: &str = "Nice to meet you ";    // The &str type is a string slice, which is a reference to a string. It is a primitive type in Rust, and it is immutable.

    // Now for us to be able to receive input from the user, we need to use the io library
    io::stdin().read_line(&mut name).expect("Failed to receive input");  // The read_line method takes a mutable reference to a string. The expect method is a method of the Result type, which is an enum that represents either success or failure. If the Result is an Err variant, the expect method will crash the program and display the message that you pass to it as an argument.
    println!("Hello {}! {}", name.trim_end(), greeting);  // The trim method removes any whitespace from the beginning and end of the string.
}
