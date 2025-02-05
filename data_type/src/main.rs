// To stop the compiler from showing warnings, we can use the following command:
#![allow(unused_variables)]

// Lets add some library to our project
use std::io;        // Another way to import everything from the library is to use the wildcard operator * like this use std::*;
use rand::Rng;      // If you add the library rand and gives you a warning, you can add the following line to the Cargo.toml file under the dependencies section: rand = "0.8.4"

// You can add many libraries at once by separating them with a comma like this: use std::io:: {self, Write};
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main(){
    // Rust has two main categories of data types: scalar and compound.
    // Scalar types represent a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.
    // Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

    // Integers
    // An integer is a number without a fractional component. Rust has signed and unsigned integers. 
    // Signed integers can be positive, negative, or zero. Unsigned integers can only be positive or zero.
    // Rust has 8, 16, 32, 64, and 128-bit signed and unsigned integers. The isize and usize depend on the kind of computer your program is running on: 64 bits if you're on a 64-bit architecture and 32 bits if you're on a 32-bit architecture.
    let x: i32 = 10;  // i32 is a signed integer with 32 bits.
    let y: u32 = 20;  // u32 is an unsigned integer with 32 bits.
    let z: isize = 30;  // isize is a signed integer with 64 bits.
    let w: usize = 40;  // usize is an unsigned integer with 64 bits.

    // Floating-Point Numbers
    // Rust has two primitive types for floating-point numbers: f32 and f64. The default type is f64 because on modern CPUs it's roughly the same speed as f32 but is capable of more precision.
    let a: f32 = 1.0;  // f32 is a floating-point number with 32 bits.
    let b: f64 = 2.0;  // f64 is a floating-point number with 64 bits.

    // Booleans
    // Booleans represent one of two values: true or false.
    let c: bool = true;
    let d: bool = false;

    // Characters
    // Rust's char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than ASCII.
    let e: char = 'a';
    let f: char = '😻';

    // Compound Types
    // Tuples
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length

    // Lets get the maximum size of a u32 integer
    println!("The maximum size of a u32 integer is: {}", u32::MAX);
    println!("The maximum size of a u64 integer is: {}", u64::MAX);
    println!("The maximum size of a usize integer is: {}", usize::MAX);
    println!("The maximum size of a u128 integer is: {}", u128 ::MAX);
    println!("The maximum size of a f32 integer is: {}", f32::MAX);
    println!("The maximum size of a f64 integer is: {}", f64::MAX);


    // Lets check the precision of a f32 and f64 floating-point numbers
    let g: f32 = 1.111111111111111;
    let h: f64 = 0.111111111111111;

    println!("f32:{}", g + 1.0);
    println!("f64:{}", h + 1.0);

    // Lets generate a random number between 1 and 100
    let random_num: i32 = rand::thread_rng().gen_range(1..101);
    println!("Random number: {}", random_num);
}