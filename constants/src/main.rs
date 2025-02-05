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
    const ONE_MIL: u32 = 1_000_000;  // Constants are always immutable, and their type must be annotated. The naming convention for constants is to use all uppercase with underscores between words.
    const PI: f32 = 3.14159;
    // Rust accept you to declare multiple variables with the same name, but with different types.
    let age: &str = "34";
    let mut age: u32 =  age.trim().parse().expect("Age wasn't assigned a number");  // You can shadow a variable by using the same name and changing the type or value.

    age = age + 1;
    println!("I'm {} and I want $ {}", age, ONE_MIL);
}
