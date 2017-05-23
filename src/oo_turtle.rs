use common::*;

#[derive(Debug, Default)]
pub struct Turtle {
    current_position: Position,
    current_angle: Angle,
    current_color: PenColor,
    current_pen_state: PenState
}


impl Turtle {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn move_forward(&mut self, distance: Distance) {
        log(format!("Move {:0.1}", distance));
        let new_position = calc_new_position(distance, self.current_angle, self.current_position);
        if self.current_pen_state == PenState::Down {
            dummy_draw_line(log, self.current_position, new_position, self.current_color);
        }
        self.current_position = new_position;
    }

    pub fn turn(&mut self, angle: Angle) {
        log(format!("Turn {:0.1}", angle));
        let new_angle = (self.current_angle + angle) % 360.0;
        self.current_angle = new_angle;
    }
}