use common::*;

#[derive(Debug)]
pub struct Turtle {
    current_position: Position,
    current_angle: Angle,
    current_color: PenColor,
    current_pen_state: PenState,
    log: fn(String)
}


impl Turtle {
    pub fn new(log: fn(String)) -> Self {
        Turtle {
            current_position: Default::default(),
            current_angle: Default::default(),
            current_color: Default::default(),
            current_pen_state: Default::default(),
            log
        }
    }

    pub fn move_forward(&mut self, distance: Distance) {
        (self.log)(format!("Move {:0.1}", distance));
        let new_position = calc_new_position(distance, self.current_angle, self.current_position);
        if self.current_pen_state == PenState::Down {
            dummy_draw_line(self.log, self.current_position, new_position, self.current_color);
        }
        self.current_position = new_position;
    }

    pub fn turn(&mut self, angle: Angle) {
        (self.log)(format!("Turn {:0.1}", angle));
        let new_angle = (self.current_angle + angle) % 360.0;
        self.current_angle = new_angle;
    }

    pub fn pen_up(&mut self) {
        (self.log)("Pen up".to_string());
        self.current_pen_state = PenState::Up;
    }

    pub fn pen_down(&mut self) {
        (self.log)("Pen up".to_string());
        self.current_pen_state = PenState::Down;
    }

    pub fn set_color(&mut self, color: PenColor) {
        (self.log)(format!("SetColor {:?}", color));
        self.current_color = color;
    }
}