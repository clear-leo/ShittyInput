This is a very shitty input managing crate, I made it because
I was tired of writing three or four lines per each input.

The whole crate was basically just made to "shorten your code cuh!"

I don't know how to write docs soooo, use cases:
 - ```shittyinput::int()``` returns a value of size ```isize```
 - ```shittyinput::string()``` returns a string
 - ```shittyinput::float``` has two cases, one of size ```f32``` and
another of size ```f64```. To use them do ```shittyinput::float::f32()``` or ```shittyinput::float::f64()```
All of the functions return a Result with either the intended value or the error.

Here's the code in [github](https://github.com/clear-leo/ShittyInput)
