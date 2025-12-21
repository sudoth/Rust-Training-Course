// This chapter is dedicated to the common programming concepts, like variables and their
// mutability, data types, functions and control flow stuff

// MUTABILITY
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function that declares a mutable integer variable, assigns it the value 5, then changes
// it to 10, and prints both values.

#[allow(dead_code)]
pub fn simple_mutability() {
    let mut x: i32 = 5;
    println!("Initial value of x: {}", x);

    x = 10;
    println!("New value of x: {}", x);
}

// DATA TYPES
// ================================================================================================

// ----- 2 --------------------------------------
// Create variables of types `i32``, `f64``, `bool``, and `char``, assign them values, and print
// them.

#[allow(dead_code)]
pub fn simple_data_types() {
    let a: i32 = 42;
    let b: f64 = 3.14159;
    let c: bool = true;
    let d: char = 'R';

    println!("i32: {}", a);
    println!("f64: {}", b);
    println!("bool: {}", c);
    println!("char: {}", d);
}

// FUNCTIONS
// ================================================================================================

// ----- 3 --------------------------------------
// Write a function `square` that takes a `u32` integer and returns its square as `u32`.

pub fn square(n: u32) -> u32 {
    n * n
}

// ----- 4 --------------------------------------
// Write a recursive function `factorial` that computes the factorial of a number (n!) as `u32`.

pub fn factorial(n: u32) -> u32 {
    if n == 0 || n == 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// CONTROL FLOW
// ================================================================================================

// ----- 5 --------------------------------------
// Write a program that prints whether a provided signed integer number is positive, negative, or
// zero using `if` statement.

pub fn sign_checker(number: i32) -> &'static str {
    if number > 0 {
        "positive"
    } else if number < 0 {
        "negative"
    } else {
        "zero"
    }
}

// ----- 6 --------------------------------------
// Write a program that finds the largest number in an array of 5 integers using a for or while
// loop.

pub fn find_biggest_number(some_array: [u32; 5]) -> u32 {
    let mut max_val = 0;

    for &number in some_array.iter() {
        if number > max_val {
            max_val = number;
        }
    }

    max_val
}