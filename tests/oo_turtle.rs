extern crate rusty_13_ways_of_looking_at_a_turtle;

use rusty_13_ways_of_looking_at_a_turtle::common::log;
use rusty_13_ways_of_looking_at_a_turtle::oo_turtle::Turtle;

#[test]
fn test_create() {
    let turtle = Turtle::new(log);
    println!("created turtle: {:?}", turtle);
}