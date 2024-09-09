This is a *fairly* shitty input managing crate, I made it because
 I was tired of writing three or four lines per each input.
 
 The whole crate was basically just made to "shorten your code cuh!"
 
 The crate currently contains:
 - ```get_int()``` Returns an integer of size ```isize```. Returns a standard ```io::Error``` if it couldn't parse the input.
 - ```get_string()``` Returns the inputed String, no need for error handling there.
 - ```get_f32()``` Returns a float of type ```f32```. Returns a standard ```io::Error``` if it couldn't parse the input.
 - ```get_f64()``` Returns a float of type ```f64```. Returns a standard ```io::Error``` if it couldn't parse the input.
 
 1.0.0 release babyyy!!