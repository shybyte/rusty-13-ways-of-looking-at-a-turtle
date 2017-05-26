extern crate rusty_13_ways_of_looking_at_a_turtle;

use std::str::FromStr;

use rusty_13_ways_of_looking_at_a_turtle::oo_turtle::Turtle;
use rusty_13_ways_of_looking_at_a_turtle::common::*;

#[derive(Debug)]
struct TurtleApiLayer {
    turtle: Turtle
}

impl TurtleApiLayer {
    pub fn new() -> TurtleApiLayer {
        TurtleApiLayer { turtle: Turtle::new(log) }
    }

    /// convert the distance parameter to a float, or throw an exception
    pub fn validate_distance(distance_str: &str) -> Distance {
        FromStr::from_str(distance_str)
            .map_err(|err| format!("Invalid distance '{:}' [{:}]", distance_str, err))
            .unwrap()
    }

    /// convert the angle parameter to a float, or throw an exception
    pub fn validate_angle(angle_str: &str) -> Angle {
        FromStr::from_str(angle_str)
            .map_err(|err| format!("Invalid angle '{:}' [{:}]", angle_str, err))
            .unwrap()
    }

    /// convert the color parameter to a PenColor, or throw an exception
    pub fn validate_color(color_str: &str) -> PenColor {
        match color_str {
            "Black" => PenColor::Black,
            "Blue" => PenColor::Blue,
            "Red" => PenColor::Red,
            _ => panic!("Color '{:}' is not recognized", color_str)
        }
    }

    pub fn exec(&mut self, command_str: &str) {
        let tokens: Vec<_> = command_str.split_whitespace().collect();
        match (tokens.get(0), tokens.get(1)) {
            (Some(&"Move"), Some(distance_str)) => {
                let distance = TurtleApiLayer::validate_distance(distance_str);
                self.turtle.move_forward(distance);
            }
            (Some(&"Turn"), Some(angle_str)) => {
                let angle = TurtleApiLayer::validate_angle(angle_str);
                self.turtle.turn(angle);
            }
            (Some(&"SetColor"), Some(color_str)) => {
                let color = TurtleApiLayer::validate_color(color_str);
                self.turtle.set_color(color);
            }
            (Some(&"Pen"), Some(&"Down")) => self.turtle.pen_down(),
            (Some(&"Pen"), Some(&"Up")) => self.turtle.pen_up(),
            _ => panic!("Unknown command {:}", command_str)
        }
    }
}


fn draw_triangle() {
    println!("Start drawing triangle");

    let mut api = TurtleApiLayer::new();

    api.exec("Move 100");
    api.exec("Turn 120");
    api.exec("Move 100");
    api.exec("Turn 120");
    api.exec("Move 100");
    api.exec("Turn 120");

    println!("turtle = {:?} \n", api);
    // back home at (0,0) with angle 0
}

fn draw_three_lines() {
    println!("Start drawing 3 lines");

    let mut api = TurtleApiLayer::new();

    // draw black line
    api.exec("Pen Down");
    api.exec("SetColor Black");
    api.exec("Move 100");

    // move without drawing
    api.exec("Pen Up");
    api.exec("Turn 90");
    api.exec("Move 100");
    api.exec("Turn 90");

    // draw red line
    api.exec("Pen Down");
    api.exec("SetColor Red");
    api.exec("Move 100");

    // move without drawing
    api.exec("Pen Up");
    api.exec("Turn 90");
    api.exec("Move 100");
    api.exec("Turn 90");

    // back home at (0,0) with angle 0
    // draw diagonal blue line
    api.exec("Pen Down");
    api.exec("SetColor Blue");
    api.exec("Turn 45");
    api.exec("Move 100");

    println!("back home turtle = {:?}", api);
}


fn draw_polygon(n: usize) {
    println!("Start drawing polygon");

    let angle = 180.0 - (360.0 / n as f64);
    let mut api = TurtleApiLayer::new();

    for _ in 0..n {
        api.exec("Move 100.0");
        api.exec(&format!("Turn {:}", angle));
    }

    println!("final turtle = {:?}\n", api);
}

#[allow(dead_code)]
fn trigger_errors() {
    let mut api = TurtleApiLayer::new();
    api.exec("Move bad");
}

fn main() {
    draw_triangle();
    draw_three_lines();
    draw_polygon(4);
    // trigger_errors();
}
