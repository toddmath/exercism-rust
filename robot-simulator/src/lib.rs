// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_left(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

pub struct Robot {
    position: (i32, i32),
    facing: Direction,
}

impl Robot {
    pub const fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            position: (x, y),
            facing: d,
        }
    }

    pub fn turn_right(self) -> Self {
        Self {
            facing: self.facing.turn_right(),
            position: self.position,
        }
    }

    pub fn turn_left(self) -> Self {
        Self {
            facing: self.facing.turn_left(),
            position: self.position,
        }
    }

    pub fn advance(self) -> Self {
        let (x, y) = self.position;
        let (x2, y2) = match self.facing {
            Direction::North => (x, y + 1),
            Direction::East => (x + 1, y),
            Direction::South => (x, y - 1),
            Direction::West => (x - 1, y),
        };
        Self {
            position: (x2, y2),
            facing: self.facing,
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut clone = self;
        for i in instructions.chars() {
            clone = match i {
                'A' => clone.advance(),
                'L' => clone.turn_left(),
                'R' => clone.turn_right(),
                _ => panic!("Invalid instruction"),
            }
        }
        clone
    }

    pub const fn position(&self) -> (i32, i32) {
        self.position
    }

    pub const fn direction(&self) -> &Direction {
        &self.facing
    }
}
