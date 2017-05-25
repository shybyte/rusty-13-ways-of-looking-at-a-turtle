extern crate rusty_13_ways_of_looking_at_a_turtle;

use rusty_13_ways_of_looking_at_a_turtle::fp_turtle;
use rusty_13_ways_of_looking_at_a_turtle::fp_utils::*;
use rusty_13_ways_of_looking_at_a_turtle::fp_turtle::{Turtle, initial_turtle_state};
use rusty_13_ways_of_looking_at_a_turtle::common::*;
use rusty_13_ways_of_looking_at_a_turtle::common::PenColor::*;


// versions of the turtle function with log baked in (via partial application)

fn move_forward(distance: Distance, turtle: &Turtle) -> Turtle {
    fp_turtle::move_forward(log, distance, turtle)
}

fn turn(distance: Angle, turtle: &Turtle) -> Turtle {
    fp_turtle::turn(log, distance, turtle)
}

fn set_color(color: PenColor, turtle: &Turtle) -> Turtle {
    fp_turtle::set_color(log, color, turtle)
}

fn pen_up(turtle: &Turtle) -> Turtle {
    fp_turtle::pen_up(log, turtle)
}

fn pen_down(turtle: &Turtle) -> Turtle {
    fp_turtle::pen_down(log, turtle)
}


fn draw_triangle() {
    println!("Start drawing triangle");

    let turtle0 = initial_turtle_state();
    let turtle1 = move_forward(100.0, &turtle0);
    let turtle2 = turn(120.0, &turtle1);
    let turtle3 = move_forward(100.0, &turtle2);
    let turtle4 = turn(120.0, &turtle3);
    let turtle5 = move_forward(100.0, &turtle4);
    let final_turtle = turn(120.0, &turtle5);

    // back home at (0,0) with angle 0
    println!("final_turtle = {:?} \n", final_turtle);
}

fn draw_triangle_pipe_line() {
    println!("Start drawing triangle with pipeline");

    let final_turtle = pipeline(initial_turtle_state(), &[
        p1(move_forward, 100.0),
        p1(turn, 120.0),
        p1(move_forward, 100.0),
        p1(turn, 120.0),
        p1(move_forward, 100.0),
        p1(turn, 120.0),
    ]);

    // back home at (0,0) with angle 0
    println!("final_turtle = {:?} \n", final_turtle);
}


fn draw_three_line_pipe_line() {
    println!("Start drawing three lines with pipeline");

    let final_turtle = pipeline(initial_turtle_state(), &[
        // black line
        p0(pen_down),
        p1(set_color, Black),
        p1(move_forward, 100.0),
        // move without drawing
        p0(pen_up),
        p1(turn, 90.0),
        p1(move_forward, 100.0),
        p1(turn, 90.0),
        // red line
        p0(pen_down),
        p1(set_color, Red),
        p1(move_forward, 100.0),
        // move without drawing
        p0(pen_up),
        p1(turn, 90.0),
        p1(move_forward, 100.0),
        p1(turn, 90.0),
    ]);

    // back home at (0,0) with angle 0
    println!("final_turtle = {:?} \n", final_turtle);
}

fn draw_polygon(n: usize) {
    println!("Start drawing polygon");

    let angle = 180.0 - (360.0 / n as f64);

    let one_side = |turtle: Turtle, _| {
        turn(angle, &move_forward(100.0, &turtle))
    };

    let final_turtle = (0..n).fold(initial_turtle_state(), one_side);
    println!("final_turtle = {:?} \n", final_turtle);
}


fn main() {
    draw_triangle();
    draw_triangle_pipe_line();
    draw_three_line_pipe_line();
    draw_polygon(4);
}
