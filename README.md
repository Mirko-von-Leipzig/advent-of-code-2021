# advent-of-code-2021
Solving [Advent of Code 2021](https://adventofcode.com/2021) using Rust.

The focus is not on execution speed but rather on learning by building understandable abstractions. A good example of this is implementing custom iterator types.

## Project layout

The project is structured as a library with each day's solutions in its own module as `day_xx`. Supporting utility code is also contained within the library. 

Of note is that the solutions themselves are implemented as tests -- this lets us use basic CI to check that any changes to utility functions don't break previous solutions.
