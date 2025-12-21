// This chapter is dedicated to the object oriented programming features of Rust.

use std::fmt;

// DYNAMIC DISPATCH
// ================================================================================================

// ----- 1 --------------------------------------
// Design a small simulation of a zoo where you have different animal types that can make noise and
// move. You have to use dynamic dispatch (trait objects) so that a collection can hold a mix of 
// different types of animals and call methods uniformly.

pub trait Animal {
    fn name(&self) -> &str;
    fn make_noise(&self) -> String;
    fn move_position(&mut self, delta_x: f64, delta_y: f64);
    fn position(&self) -> (f64, f64);
}

pub struct Lion {
    name: String,
    x: f64,
    y: f64,
}

impl Lion {
    pub fn new(name: &str, x: f64, y: f64) -> Self {
        Self { name: name.to_string(), x, y }
    }
}

impl Animal for Lion {
    fn name(&self) -> &str {
        &self.name
    }
    fn make_noise(&self) -> String {
        format!("{}: Roar!", self.name)
    }
    fn move_position(&mut self, delta_x: f64, delta_y: f64) {
        self.x += delta_x;
        self.y += delta_y;
    }
    fn position(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}

pub struct BritishPigeon {
    name: String,
    x: f64,
    y: f64,
}

impl BritishPigeon {
    pub fn new(name: &str, x: f64, y: f64) -> Self {
        Self { name: name.to_string(), x, y }
    }
}

impl Animal for BritishPigeon {
    fn name(&self) -> &str {
        &self.name
    }
    fn make_noise(&self) -> String {
        format!("{}: Oi mate! Bloody hell I love fish'n'chips brof!'", self.name)
    }
    fn move_position(&mut self, delta_x: f64, delta_y: f64) {
        self.x += delta_x * 2.0;
        self.y += delta_y * 2.0;
    }
    fn position(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}


pub struct Zoo {
    animals: Vec<Box<dyn Animal>>,
}

impl Zoo {
    pub fn new() -> Self {
        Zoo { animals: Vec::new() }
    }

    pub fn add_animal(&mut self, animal: Box<dyn Animal>) {
        self.animals.push(animal);
    }

    pub fn make_all_noises(&self) -> Vec<String> {
        self.animals.iter()
            .map(|animal| animal.make_noise())
            .collect()
    }

    pub fn move_all(&mut self, delta_x: f64, delta_y: f64) {
        self.animals.iter_mut()
            .for_each(|animal| animal.move_position(delta_x, delta_y));
    }

    pub fn positions(&self) -> Vec<(&str, (f64, f64))> {
        self.animals.iter()
            .map(|animal| (animal.name(), animal.position()))
            .collect()
    }
}

pub fn run_zoo_simulation() -> (Vec<String>, Vec<(&str, (f64, f64))>) {
    let mut london_zoo = Zoo::new();

    london_zoo.add_animal(Box::new(Lion::new("Leo", 0.0, 0.0)));
    london_zoo.add_animal(Box::new(BritishPigeon::new("Chippy", 5.0, 5.0)));
    london_zoo.add_animal(Box::new(Lion::new("Nala", -10.0, 10.0)));

    let noises = london_zoo.make_all_noises();

    london_zoo.move_all(1.0, -1.0);

    let positions = london_zoo.positions();
    
    (noises, positions)
}


// SUPERTRAITS
// ================================================================================================

// ----- 2 --------------------------------------
// Implement the `BackTo2007` trait with the `std::fmt::Display` as a supertrait for it. Implement 
// `BackTo2007` trait for the `Account` struct which consists of `name: String` and 
// `year_of_birth: u32` fields.
//
// This `BackTo2007` trait should have just one `cringify(&self) -> String` method, which will
// make the account much more cringy by adding "★彡Xx_" to the left of the `self.to_string()` and 
// "_xX彡★" to the right. Just like that: ★彡Xx_NAGIBATOR1999_xX彡★
//
// Notice that you also should decide how to display the account.


pub struct Account {
    name: String,
    year_of_birth: u32,
}

impl Account {
    pub fn new(name: &str, year_of_birth: u32) -> Self {
        Self { name: name.to_string(), year_of_birth }
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.name.to_uppercase(), self.year_of_birth)
    }
}

pub trait BackTo2007: fmt::Display {
    fn cringify(&self) -> String {
        format!("★彡Xx_{}_xX彡★", self.to_string())
    }
}

impl BackTo2007 for Account {}

pub fn run_supertrait_example() -> String {
    let account = Account::new("nagibator", 1999);
    account.cringify()
}


// DEFAULT GENERIC TYPE PARAMETERS AND ASSOCIATED TYPES
// ================================================================================================

// ----- 3 --------------------------------------
// Implement a `Converter` trait with `Input` and `Output` associated types. `Input` should have a 
// `String` default type. This trait should have a `convert` method which takes a value of type 
// `Input` and returns a value of type `Output`.

pub trait Converter {
    type Input = String; 
    type Output;

    fn convert(input: Self::Input) -> Self::Output;
}

pub struct StringToIntConverter;

impl Converter for StringToIntConverter {
    type Output = i32;

    fn convert(input: String) -> Self::Output {
        input.parse().unwrap_or(0)
    }
}

pub struct IntToHexConverter;

impl Converter for IntToHexConverter {
    type Input = i32; 
    type Output = String;

    fn convert(input: Self::Input) -> Self::Output {
        format!("{:X}", input)
    }
}

pub fn run_converter_examples() -> (i32, String) {
    let i_result = StringToIntConverter::convert("42".to_string());
    let h_result = IntToHexConverter::convert(255);
    
    (i_result, h_result)
}