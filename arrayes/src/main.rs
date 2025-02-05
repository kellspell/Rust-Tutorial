// To stop the compiler from showing warnings, we can use the following command:
#![allow(unused_variables)]

// Lets add some library to our project
use std::io;        // Another way to import everything from the library is to use the wildcard operator * like this use std::*;
use rand::Rng;      // If you add the library rand and gives you a warning, you can add the following line to the Cargo.toml file under the dependencies section: rand = "0.8.4"

// You can add many libraries at once by separating them with a comma like this: use std::io:: {self, Write};
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// In tis lesson we're going to learn about arrays in Rust
// Arrays are fixed size in Rust, once you declare an array with a certain size, you can't change it
// elements in an array must be of the same type
// Arrays are zero-indexed, which means that the first element in an array is at index 0, the second element is at index 1, and so on.
fn main() {
   let arr_1: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let arr_2: [i32; 5] = [0; 5];
    println!("arr_1: {:?}", arr_1[0]);
    println!("Length: {:?}", arr_2.len());

    // We can also loop over an array
    let mut loop_idx: usize = 0;
    loop {
        println!("Current value: {}", arr_1[loop_idx]); // Print current value
        
        if arr_1[loop_idx] % 9 == 0 {
            loop_idx += 1;
            continue;
        }
        if arr_1[loop_idx] == 9 {
            break;
        }
        loop_idx += 1; // Increment index in main loop
        
        // Safety check to prevent going past array bounds
        if loop_idx >= arr_1.len() {
            break;
        }
    }

    println!("The value of loop_idx is: {}", loop_idx);

    /*
    Using for loop (more idiomatic)
        for (idx, &value) in arr_1.iter().enumerate() {
            println!("Value at index {}: {}", idx, value);
            if value == 9 {
                break;
                }
            }
    */

    // While loop
    let mut while_idx: usize = 0;
    while while_idx < arr_1.len() {
        println!("Value at index {}: {}", while_idx, arr_1[while_idx]);
        if arr_1[while_idx] == 9 {
            break;
        }
        while_idx += 1;
    }

    println!("The value of while_idx is: {}", while_idx);
}
