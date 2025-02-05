// To stop the compiler from showing warnings, we can use the following command:
#![allow(unused_variables)]

// Lets add some library to our project
use std::io;        // Another way to import everything from the library is to use the wildcard operator * like this use std::*;
use rand::Rng;      // If you add the library rand and gives you a warning, you can add the following line to the Cargo.toml file under the dependencies section: rand = "0.8.4"

// You can add many libraries at once by separating them with a comma like this: use std::io:: {self, Write};
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// Here in this lesson, we are going to learn about casting in Rust

fn main() {
    // Lets say you want to cast two numbers
    let x: u8 = 5;
    let y: u8 = 10;

    // You can cast them like this
    let z: u16 = x as u16 + y as u16;
    println!("The value of z is: {}", z);

    // Lets also work with enums 
    enum Days {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    impl Days {
        fn day_number(&self) -> u8 {
            match self {
                Days::Monday => 1,
                Days::Tuesday => 2,
                Days::Wednesday => 3,
                Days::Thursday => 4,
                Days::Friday => 5,
                Days::Saturday => 6,
                Days::Sunday => 7,
            }
        }
    }

    let day = Days::Tuesday;
    let day_number = day.day_number();
    println!("The day number is: {}", day_number);
}
