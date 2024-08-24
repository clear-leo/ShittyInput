//! This is a very shitty input managing crate, I made it because
//! I was tired of writing three or four lines per each input.
//! 
//! The whole crate was basically just made to "shorten your code cuh!"
//! 
//! I don't know how to write docs soooo, use cases:
//!  - ```shittyinput::int()``` returns a value of size ```isize```
//!  - ```shittyinput::string()``` returns a string
//!  - ```shittyinput::float``` has two cases, one of size ```f32``` and
//! another of size ```f64```. To use them do ```shittyinput::float::f32()``` or ```shittyinput::float::f64()```
//! 
//! All of the functions return a Result with either the intended value or the error.
//! 
//! Here's the code in [github](https://github.com/clear-leo/ShittyInput)


use std::{io, num::ParseIntError};

pub fn int() -> Result<isize, ParseIntError> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error on terminal input (ShittyInput crate!");
    let result = match input.trim().parse::<isize>() {
        Ok(result) => Ok(result),
        Err(error) => Err(error)
    };
    result
}

pub fn string() -> String {
    let mut result = String::new();
    io::stdin().read_line(&mut result).expect("Error on terminal input (ShittyInput crate!");
    let result = result.trim();
    result.to_string()
}

pub struct float {
    
}

impl float {
    pub fn f32() -> Result<f32, std::num::ParseFloatError> {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error on terminal input (ShittyInput crate!");
        let result = match input.trim().parse::<f32>() {
            Ok(result) => Ok(result),
            Err(error) => Err(error)
        };
        result
    }

    pub fn f64() -> Result<f64, std::num::ParseFloatError> {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error on terminal input (ShittyInput crate!");
        let result = match input.trim().parse::<f64>() {
            Ok(result) => Ok(result),
            Err(error) => Err(error)
        };
        result
    }
}