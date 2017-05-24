use std::marker::PhantomData;
use common::*;

pub enum PenDownState {}

pub enum PenUpState {}


#[derive(Debug)]
pub struct Turtle<PenPhantomState>
{
    current_position: Position,
    current_angle: Angle,
    current_color: PenColor,
    phantom_pen_state: PhantomData<PenPhantomState>
}

impl Turtle<PenDownState> {
    pub fn new() -> Self {
        Turtle {
            current_position: initial_position(),
            current_angle: 0.0,
            current_color: initial_color(),
            phantom_pen_state: PhantomData
        }
    }

    pub fn pen_up(self) -> Turtle<PenUpState> {
        log("Pen up");
        Turtle {
            phantom_pen_state: PhantomData,
            current_position: self.current_position,
            current_angle: self.current_angle,
            current_color: self.current_color,
        }
    }

    pub fn move_forward(&mut self, distance: Distance) {
        log(format!("Move {:0.1}", distance));
        let new_position = calc_new_position(distance, self.current_angle, self.current_position);
        dummy_draw_line(log, self.current_position, new_position, self.current_color);
        self.current_position = new_position;
    }
}

impl Turtle<PenUpState> {
    pub fn pen_down(self) -> Turtle<PenDownState> {
        log("Pen down");
        Turtle {
            phantom_pen_state: PhantomData,
            current_position: self.current_position,
            current_angle: self.current_angle,
            current_color: self.current_color,
        }
    }

    pub fn move_forward(&mut self, distance: Distance) {
        log(format!("Move {:0.1}", distance));
        let new_position = calc_new_position(distance, self.current_angle, self.current_position);
        self.current_position = new_position;
    }
}


impl<PenPhantomState> Turtle<PenPhantomState> {
    pub fn turn(&mut self, angle: Angle) {
        log(format!("Turn {:0.1}", angle));
        let new_angle = (self.current_angle + angle) % 360.0;
        self.current_angle = new_angle;
    }

    pub fn set_color(&mut self, color: PenColor) {
        log(format!("SetColor {:?}", color));
        self.current_color = color;
    }
}