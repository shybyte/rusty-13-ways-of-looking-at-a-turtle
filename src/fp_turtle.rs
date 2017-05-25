use common::*;

#[derive(Debug, Default)]
pub struct Turtle {
    current_position: Position,
    current_angle: Angle,
    current_color: PenColor,
    current_pen_state: PenState,
}


pub fn initial_turtle_state() -> Turtle {
    Default::default()
}

pub fn move_forward(log: fn(String), distance: Distance, turtle: &Turtle) -> Turtle {
    log(format!("Move {:0.1}", distance));
    let new_position = calc_new_position(distance, turtle.current_angle, turtle.current_position);
    if turtle.current_pen_state == PenState::Down {
        dummy_draw_line(log, turtle.current_position, new_position, turtle.current_color);
    }
    Turtle {
        current_position: new_position,
        ..*turtle
    }
}

pub fn turn(log: fn(String), angle: Angle, turtle: &Turtle) -> Turtle {
    log(format!("Turn {:0.1}", angle));
    let new_angle = (turtle.current_angle + angle) % 360.0;
    Turtle {
        current_angle: new_angle,
        ..*turtle
    }
}

pub fn pen_up(log: fn(String), turtle: &Turtle) -> Turtle {
    log("Pen up".to_string());
    Turtle {
        current_pen_state: PenState::Up,
        ..*turtle
    }
}

pub fn pen_down(log: fn(String), turtle: &Turtle) -> Turtle {
    log("Pen down".to_string());
    Turtle {
        current_pen_state: PenState::Down,
        ..*turtle
    }
}

pub fn set_color(log: fn(String), color: PenColor, turtle: &Turtle) -> Turtle {
    log(format!("SetColor {:?}", color));
    Turtle {
        current_color: color,
        ..*turtle
    }
}