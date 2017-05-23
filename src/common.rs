use std::f64::consts::PI;

type Distance = f64;
type Angle = f64;

#[derive(Debug, Copy, Clone)]
pub enum PenState {
    Up,
    Down
}

#[derive(Debug, Copy, Clone)]
pub enum PenColor {
    Black,
    Red,
    Blue
}

#[derive(Debug, Copy, Clone)]
pub struct Position {
    x: f64,
    y: f64
}


/// round a float to two places to make it easier to read
pub fn round2(flt: f64) -> f64 {
    (flt * 100.0).round() / 100.0
}

pub fn calc_new_position(distance: Distance, angle: Angle, current_pos: Position) -> Position {
    let angle_in_rads = angle * (PI / 180.0) * 1.0;
    let x1 = current_pos.x + distance * angle_in_rads.cos();
    let y1 = current_pos.y + distance * angle_in_rads.sin();
    Position { x: round2(x1), y: round2(y1) }
}

pub fn initial_position() -> Position {
    Position { x: 0.0, y: 0.0 }
}

pub fn initial_color() -> PenColor {
    PenColor::Black
}

pub fn initial_pen_state() -> PenState {
    PenState::Down
}

/// Emulating a real implementation for drawing a line
pub fn dummy_draw_line(log: fn(String), old_pos: Position, new_pos: Position, color: PenColor) {
    log(format!("...Draw line from ({:.1},{:.1}) to ({:.1},{:.1}) using {:?}", old_pos.x, old_pos.y, new_pos.x, new_pos.y, color))
}

pub fn trim_string(s: &str) -> String {
    s.trim().to_string()
}

