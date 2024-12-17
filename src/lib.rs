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