#[derive(Debug, Default)]
pub struct Turtle {
    current_position: f64
}


impl Turtle {
    pub fn new() -> Self {
        Turtle { current_position: 10.0 }
    }
}