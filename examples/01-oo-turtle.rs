extern crate rusty_13_ways_of_looking_at_a_turtle;

use rusty_13_ways_of_looking_at_a_turtle::oo_turtle::Turtle;

fn main() {
    let turtle = Turtle::new();
    println!("created turtle: {:?}", turtle);
}