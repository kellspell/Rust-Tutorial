// To stop the compiler from showing warnings, we can use the following command:
#![allow(unused_variables)]

// Lets add some library to our project
use std::io;        // Another way to import everything from the library is to use the wildcard operator * like this use std::*;
use rand::Rng;      // If you add the library rand and gives you a warning, you can add the following line to the Cargo.toml file under the dependencies section: rand = "0.8.4"

// You can add many libraries at once by separating them with a comma like this: use std::io:: {self, Write};
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// In tis lesson we're going to learn about if expressions in Rust

fn main() {
    let age: i32 = 8;
    if (age >= 1) && (age <= 5) {
        println!("Go to kindergarten");
    } else if (age > 5) && (age <= 18) {
        println!("Go to grade school");
    } else if (age > 18) && (age <= 21) {
        println!("Go to college");
    } else {
        println!("Do whatever you want");
    }

    // If you want to simulate an ternary operator in Rust you can do it like this:
    let my_age: i32 = 47;
    let can_vote = if my_age >= 18 {
        true
    } else {
        false};
    println!("Can vote: {}", can_vote);

    // Another import feature of Rust is the match expression
    let age2: i32 = 8;
    match age2 {
        1..=18 => println!("Go to school"),
        21 | 50 => println!("Go to college"),
        65..=i32::MAX => println!("Go to college"),
        _=> println!("Do whatever you want"),
    };

    // Lets see how to use match with ordering
    let my_age2: i32 = 47;
    let voting_age = 18;
    match my_age2.cmp(&voting_age) {
        Ordering::Less => println!("Can't vote"),
        Ordering::Greater => println!("Can vote"),
        Ordering::Equal => println!("Can vote"),
    }
}
