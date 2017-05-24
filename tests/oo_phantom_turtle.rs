extern crate rusty_13_ways_of_looking_at_a_turtle;

use rusty_13_ways_of_looking_at_a_turtle::oo_phantom_turtle::*;

#[test]
fn test_create() {
    let mut turtle = Turtle::new();
    // turtle.pen_down(); // compile error!

    turtle.move_forward(100.0);

    let mut turtle = turtle.pen_up();
    // turtle.pen_up(); // compile error!
    turtle.move_forward(100.0);
    turtle.pen_down();
}