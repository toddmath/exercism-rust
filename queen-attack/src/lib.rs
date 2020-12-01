#[derive(Debug)]
pub struct ChessPosition(isize, isize);

#[derive(Debug)]
pub struct Queen {
    row: isize,
    col: isize,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..=7, 0..=7) => Some(Self(rank as isize, file as isize)),
            _ => None,
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self {
            row: position.0,
            col: position.1,
        }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let dx = self.row - other.row;
        let dy = self.col - other.col;
        dx == 0 || dy == 0 || dx.abs() == dy.abs()
    }
}
