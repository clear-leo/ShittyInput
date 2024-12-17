# A fairly shitty input managing crate for rust
Made this crate because inputting things in rust is incredibly not beginner friendly. This crate gives you a few functions that abstract the non beginner friendliness away.

# Usage
The crate contains the following functions:
 - ```get_int()``` Returns an integer of size ```isize```. Returns a standard ```io::Error``` if it couldn't parse the input.
 - ```get_string()``` Returns the inputed String, no need for error handling there.
 - ```get_f32()``` Returns a float of type ```f32```. Returns a standard ```io::Error``` if it couldn't parse the input.
 - ```get_f64()``` Returns a float of type ```f64```. Returns a standard ```io::Error``` if it couldn't parse the input.