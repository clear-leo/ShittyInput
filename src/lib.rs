//! This is a *fairly* shitty input managing crate, I made it because
//! I was tired of writing three or four lines per each input.
//! 
//! The whole crate was basically just made to "shorten your code cuh!"
//! 
//! The crate currently contains:
//! - ```get_int()``` Returns an integer of size ```isize```. Returns a standard ```io::Error``` if it couldn't parse the input.
//! - ```get_string()``` Returns the inputed String, no need for error handling there.
//! - ```get_f32()``` Returns a float of type ```f32```. Returns a standard ```io::Error``` if it couldn't parse the input.
//! - ```get_f64()``` Returns a float of type ```f64```. Returns a standard ```io::Error``` if it couldn't parse the input.
//! 
//! 1.0.0 release babyyy!!


use std::io;


/// Returns an ```io::Result<isize>```
pub fn get_int() -> io::Result<isize> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error on terminal input (ShittyInput crate!)");
    let result = input.trim().parse::<isize>().map_err(|error| {
        io::Error::new(io::ErrorKind::Other, error)
    });
    result
}

/// Returns a ```String```
pub fn get_string() -> String {
    let mut result = String::new();
    io::stdin().read_line(&mut result).expect("Error on terminal input (ShittyInput crate!)");
    let result = result.trim();
    result.to_string()
}

/// Returns an ```io::Result<f32>```
pub fn get_f32 () -> io::Result<f32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error on terminal input (ShittyInput crate!)");
    let result = input.trim().parse::<f32>().map_err(|error| {
        io::Error::new(io::ErrorKind::Other, error)
    });
    result
}

/// Returns an ```io::Result<f64>```
pub fn get_f64() -> io::Result<f64> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error on terminal input (ShittyInput crate!)");
    let result = input.trim().parse::<f64>().map_err(|error| {
        io::Error::new(io::ErrorKind::Other, error)
    });
    result
}