//! This is a very shitty input managing crate, I made it because
//! I was tired of writing three or four lines per each input.
//! 
//! The benefits to using this crate:
//! - Shorten yo code cuh!!!
//! 
//! The downsides to using this crate:
//! - Unoptimized code
//! - May cause errors
//! 
//! 
//! I don't know how to write docs soooo, use cases:
//! - ```shittyinput::int()``` returns an integer of size ```isize```
//! - ```shittyinput::string()``` returns a string
//! - ```shittyinput::float``` has two cases, one of size ```f32``` and
//! another of size ```f64```. To use them do ```shittyinput::float::bit32()``` or ```shittyinput::float::bit64()```
//! 
//! 
//! Here's the code in [github](https://github.com/clear-leo/ShittyInput)


use std::io;

pub fn int() -> isize {
    let mut result = String::new();
    io::stdin().read_line(&mut result);
    let result = result.trim().parse().expect("ERROR: Enter a valid number");
    result
}

pub fn string() -> String {
    let mut result = String::new();
    io::stdin().read_line(&mut result);
    let result = result.trim();
    result.to_string()
}

pub struct float {
    bit32: f32,
    bit64: f64
}

impl float {
    pub fn bit32() -> f32 {
        let mut result = String::new();
        io::stdin().read_line(&mut result);
        let result: f32 = result.trim().parse().expect("ERROR: Enter a valid (float 32-bit) number");
        result
    }

    pub fn bit64() -> f64 {
        let mut result = String::new();
        io::stdin().read_line(&mut result);
        let result: f64 = result.trim().parse().expect("ERROR: Enter a valid (float 64-bit) number");
        result
    }
}