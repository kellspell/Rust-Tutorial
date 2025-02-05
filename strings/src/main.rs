// To stop the compiler from showing warnings, we can use the following command:
#![allow(unused_variables)]

// Lets add some library to our project
use std::io;        // Another way to import everything from the library is to use the wildcard operator * like this use std::*;
use rand::Rng;      // If you add the library rand and gives you a warning, you can add the following line to the Cargo.toml file under the dependencies section: rand = "0.8.4"

// You can add many libraries at once by separating them with a comma like this: use std::io:: {self, Write};
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// In this lesson we will learn about strings in Rust
fn main() {
    // In rust there are two types of strings: String and &str
    // String is a vector of characters and it is mutable
    // &str is a reference that points to a string in memory and it is immutable
    let mut s = String::new();
    s.push_str("Hello, ");
    s.push('W');    // Change 1: Use single quotes for char, remove 'char' keyword
    
    for word in s.split_whitespace() {    // Change 2: Remove type annotation
        println!("{}", word);
    }
    
    // lets say you would like to replace a word in a string
    let s1 = s.replace("W", "there");    // Change 3: Use s instead of undefined s1
    println!("{}", s1);

    // Another way to create a string is to use the to_string() method
    let s3 = String::from("x r t b h k k a m c");
    let mut v1: Vec<&str> = s3.split_whitespace().collect();
    v1.sort();
    v1.dedup(); // Remove duplicates
    for word in v1 {
        print!("{} ", word);
    }
    println!("{}", s3);

    let s4: &str = "Random string";    // Change 4: Use &str instead of String
    let s5: String = s4.to_string();    // Change 5: Use to_string() method instead of to_string
    println!("{}", s5);

    // Converting a string into array of bytes 
    let s6 = String::from("Hello, world!");
    let s7 = s6.as_bytes();
    for byte in s7 {
        print!("{} ", byte);
    }
    println!("");
    let s8 = String::from_utf8(s7.to_vec()).unwrap();    // Change 6: Use unwrap() method instead of unwrap_or()
    println!("{}", s8);

    // If you want to delete a string, you can use the clear() method
    let mut s9 = String::from("Hello, world!");
    s9.clear();
    println!(" Deleted {}", s9);
}
