use std::fmt;

// Should be moved to a generic folder
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(origin: &Position, shift: &Position, moves: usize) -> Position {
        let mut position = Position { x: 0, y: 0 };
        position.several_shift(shift, moves);
        position.shift(origin);
        position
    }

    pub fn shift(&mut self, shift: &Position) {
        self.x += shift.x;
        self.y += shift.y;
    }
    pub fn several_shift(&mut self, shift: &Position, n_moves: usize) {
        self.x += shift.x * n_moves;
        self.y += shift.y * n_moves;
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
