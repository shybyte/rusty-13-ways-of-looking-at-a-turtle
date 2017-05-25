# Thirteen different ways of implementing a LOGO-style turtle in Rust [![Build Status](https://travis-ci.org/shybyte/rusty-13-ways-of-looking-at-a-turtle.svg?branch=master)](https://travis-ci.org/shybyte/rusty-13-ways-of-looking-at-a-turtle) 

See also [https://github.com/swlaschin/13-ways-of-looking-at-a-turtle](https://github.com/swlaschin/13-ways-of-looking-at-a-turtle) 
for the Original in F#.


## Common Code

This [common code](src/common.rs) is used from all implementations.

## The thirteen implementations

#### 1. Basic OO -- Class with mutable state

* [Example Turtle Usage](examples/01-oo-turtle.rs) 
* [Example Turtle](src/oo_turtle.rs) 

#### 2. Basic FP -- Module of functions with immutable state

* [Example Turtle Usage](examples/02-fp-turtle.rs) 
* [Example Turtle](src/fp_turtle.rs) 
* [FP Utils](src/fp_utils.rs) 

## License

MIT

## Copyright

Copyright (c) 2017 Marco Stahl
