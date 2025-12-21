// This chapter is dedicated to the ownership, borrowing and slices

// OWNERSHIP
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function `longest_owned(s1: String, s2: String) -> String` that returns the longer of
// two strings. Check that both original strings are moved into the function, and only the returned
// can still be used.
//
// You can implement the function and use it right inside the `string_ownership` function.

fn longest_owned(s1: String, s2: String) -> String {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

#[allow(dead_code)]
pub fn string_ownership() {
    let string1 = String::from("Rust Ownership");
    let string2 = String::from("Short");

    let winner = longest_owned(string1, string2); 

    println!("The longest string is: {}", winner);    
}

// BORROWING
// ================================================================================================

// ----- 2 --------------------------------------
// Write a function `print_length(s: ???)` that takes some string and prints its length without
// taking ownership. First use it with some random (censored) string, and then print this string to
// show that it was not moved and still available.
//
// You can implement the function and use it right inside the `simple_borrowing` function.

fn print_length(s: &str) {
    println!("The length of the string is: {}", s.len());
}

#[allow(dead_code)]
pub fn simple_borrowing() {
    let random_string = String::from("censored text");

    print_length(&random_string);

    println!("The original string is still available: {}", random_string);
}

// ----- 3 --------------------------------------
// Implement a function `append_and_return_length(string: ???, suffix: ???) -> usize` that borrows
// some string, appends a suffix to it, and returns the new length. Then call it multiple times
// to check that the string was borrowed, not moved.
//
// You can implement the function and use it right inside the `hard_borrowing` function.

fn append_and_return_length(string: &mut String, suffix: &str) -> usize {
    string.push_str(suffix);
    string.len()
}

#[allow(dead_code)]
pub fn hard_borrowing() {
    let mut data = String::from("hello");
    let suffix1 = " world";
    let suffix2 = " again!";

    let new_len1 = append_and_return_length(&mut data, suffix1);
    println!("Length after first append: {} ({})", new_len1, data);

    let new_len2 = append_and_return_length(&mut data, suffix2);
    println!("Length after second append: {} ({})", new_len2, data);

    println!("Final string: {}", data);
}

// SLICES
// ================================================================================================

// ----- 4 --------------------------------------
// Write a function last_word(s: &str) -> &str that returns the last word from a string slice.
// Assume words are separated by spaces.

pub fn last_word(slice: &str) -> &str {
    slice.split_whitespace().last().unwrap_or("")
}

// ----- 5 --------------------------------------
// Write a function longest_word(sentence: &str) -> &str that returns the longest word in a
// sentence (string slice). If several words have the same maximum length, return the last one.

pub fn longest_word(sentence: &str) -> &str {
    let mut longest = "";

    for word in sentence.split_whitespace() {
        if word.len() >= longest.len() {
            longest = word;
        }
    }

    longest
}