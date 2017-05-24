extern crate rusty_13_ways_of_looking_at_a_turtle;

use rusty_13_ways_of_looking_at_a_turtle::oo_turtle::Turtle;
use rusty_13_ways_of_looking_at_a_turtle::common::PenColor::*;

fn draw_triangle() {
    println!("Start drawing triangle");

    let mut turtle = Turtle::new();
    turtle.move_forward(100.0);
    turtle.turn(120.0);
    turtle.move_forward(100.0);
    turtle.turn(120.0);
    turtle.move_forward(100.0);
    turtle.turn(120.0);
    println!("turtle = {:?} \n", turtle);
    // back home at (0,0) with angle 0
}

fn draw_three_lines() {
    println!("Start drawing 3 lines");

    let mut turtle = Turtle::new();

    // draw black line
    turtle.pen_down();
    turtle.set_color(Black);
    turtle.move_forward(100.0);

    // move without drawing
    turtle.pen_up();
    turtle.turn(90.0);
    turtle.move_forward(100.0);
    turtle.turn(90.0);

    // draw red line
    turtle.pen_down();
    turtle.set_color(Red);
    turtle.move_forward(100.0);

    // move without drawing
    turtle.pen_up();
    turtle.turn(90.0);
    turtle.move_forward(100.0);
    turtle.turn(90.0);
    // back home at (0,0) with angle 0
    println!("back home turtle = {:?}", turtle);


    // draw diagonal blue line

    turtle.pen_down();
    turtle.set_color(Blue);
    turtle.turn(45.0);
    turtle.move_forward(100.0);

    println!("turtle = {:?}\n", turtle);
}


fn draw_polygon(n: usize) {
    println!("Start drawing polygon");

    let angle = 180.0 - (360.0/n as f64);
    let mut turtle = Turtle::new();

    for _ in 0..n {
        turtle.move_forward(100.0);
        turtle.turn(angle);
    }

    println!("turtle = {:?}\n", turtle);
}


fn main() {
    draw_triangle();
    draw_three_lines();
    draw_polygon(4);
}
