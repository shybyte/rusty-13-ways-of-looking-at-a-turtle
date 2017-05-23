extern crate rusty_13_ways_of_looking_at_a_turtle;

use rusty_13_ways_of_looking_at_a_turtle::oo_turtle::Turtle;

fn draw_triangle() {
    let mut turtle = Turtle::new();
    turtle.move_forward(100.0);
    turtle.turn(120.0);
    turtle.move_forward(100.0);
    turtle.turn(120.0);
    turtle.move_forward(100.0);
    turtle.turn(120.0);
    println!("turtle = {:?}", turtle);
    // back home at (0,0) with angle 0
}

fn main() {
    draw_triangle();
}