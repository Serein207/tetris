use std::clone::Clone;
pub const PIECE_COUNT: usize = 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    CYAN,
    BLUE,
    ORANGE,
    YELLOW,
    GREEN,
    PURPLE,
    RED,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Piece {
    pub color: Color,
    rotations: [[(u16, u16); 4]; 4],
}

impl Piece {
    pub fn get_shape<'a>(&'a self, rotation: usize) -> &'a [(u16, u16); 4] {
        &self.rotations[rotation]
    }
}

#[derive(Clone)]
pub struct PhysicalPiece {
    pub x: i16,
    pub y: i16,
    pub rotation: usize,
    pub piece: Piece,
}

impl PhysicalPiece {
    pub fn rotate_right(&mut self) {
        self.rotation = (self.rotation + 1) % 4;
    }

    pub fn rotate_left(&mut self) {
        self.rotation = (self.rotation + 4 - 1) % 4;
    }

    pub fn move_right(&mut self) {
        self.x += 1;
    }

    pub fn move_left(&mut self) {
        self.x -= 1;
    }

    pub fn newton(&mut self) {
        self.y += 1;
    }

    pub fn get_shape(&self) -> &[(u16, u16); 4] {
        self.piece.get_shape(self.rotation)
    }

    #[allow(dead_code)]
    pub fn get_piece<'a>(&'a self) -> &'a Piece {
        &self.piece
    }
}

pub static PIECES: [&Piece; PIECE_COUNT] = [
    &BLOCK_I, &BLOCK_J, &BLOCK_L, &BLOCK_O, &BLOCK_S, &BLOCK_T, &BLOCK_Z,
];

pub static BLOCK_I: Piece = Piece {
    color: Color::CYAN,
    rotations: [
        [(0, 1), (1, 1), (2, 1), (3, 1)],
        [(2, 0), (2, 1), (2, 2), (2, 3)],
        [(0, 2), (1, 2), (2, 2), (3, 2)],
        [(1, 0), (1, 1), (1, 2), (1, 3)],
    ],
};

pub static BLOCK_O: Piece = Piece {
    color: Color::YELLOW,
    rotations: [
        [(0, 0), (0, 1), (1, 0), (1, 1)],
        [(0, 0), (0, 1), (1, 0), (1, 1)],
        [(0, 0), (0, 1), (1, 0), (1, 1)],
        [(0, 0), (0, 1), (1, 0), (1, 1)],
    ],
};

pub static BLOCK_T: Piece = Piece {
    color: Color::PURPLE,
    rotations: [
        [(1, 0), (0, 1), (1, 1), (2, 1)],
        [(1, 0), (1, 1), (1, 2), (2, 1)],
        [(0, 1), (1, 1), (2, 1), (1, 2)],
        [(1, 0), (1, 1), (1, 2), (0, 1)],
    ],
};

pub static BLOCK_S: Piece = Piece {
    color: Color::GREEN,
    rotations: [
        [(0, 1), (1, 1), (1, 0), (2, 0)],
        [(1, 0), (1, 1), (2, 1), (2, 2)],
        [(0, 2), (1, 2), (1, 1), (2, 1)],
        [(0, 0), (0, 1), (1, 1), (1, 2)],
    ],
};

pub static BLOCK_Z: Piece = Piece {
    color: Color::RED,
    rotations: [
        [(0, 0), (1, 0), (1, 1), (2, 1)],
        [(1, 2), (1, 1), (2, 1), (2, 0)],
        [(0, 1), (1, 1), (1, 2), (2, 2)],
        [(0, 2), (0, 1), (1, 1), (1, 0)],
    ],
};

pub static BLOCK_J: Piece = Piece {
    color: Color::BLUE,
    rotations: [
        [(0, 0), (0, 1), (1, 1), (2, 1)],
        [(1, 2), (1, 1), (1, 0), (2, 0)],
        [(0, 1), (1, 1), (2, 1), (2, 2)],
        [(0, 2), (1, 2), (1, 1), (1, 0)],
    ],
};

pub static BLOCK_L: Piece = Piece {
    color: Color::ORANGE,
    rotations: [
        [(0, 1), (1, 1), (2, 1), (2, 0)],
        [(1, 0), (1, 1), (1, 2), (2, 2)],
        [(0, 2), (0, 1), (1, 1), (2, 1)],
        [(0, 0), (1, 0), (1, 1), (1, 2)],
    ],
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_right() {
        let mut piece = PhysicalPiece {
            x: 0,
            y: 0,
            rotation: 0,
            piece: BLOCK_I,
        };
        piece.rotate_right();
        assert_eq!(piece.rotation, 1);
        piece.rotate_right();
        assert_eq!(piece.rotation, 2);
        piece.rotate_right();
        assert_eq!(piece.rotation, 3);
        piece.rotate_right();
        assert_eq!(piece.rotation, 0);
    }

    #[test]
    fn test_rotate_left() {
        let mut piece = PhysicalPiece {
            x: 0,
            y: 0,
            rotation: 0,
            piece: BLOCK_I,
        };
        piece.rotate_left();
        assert_eq!(piece.rotation, 3);
        piece.rotate_left();
        assert_eq!(piece.rotation, 2);
        piece.rotate_left();
        assert_eq!(piece.rotation, 1);
        piece.rotate_left();
        assert_eq!(piece.rotation, 0);
    }

    #[test]
    fn test_move_right() {
        let mut piece = PhysicalPiece {
            x: 0,
            y: 0,
            rotation: 0,
            piece: BLOCK_I,
        };
        piece.move_right();
        assert_eq!(piece.x, 1);
        piece.move_right();
        assert_eq!(piece.x, 2);
    }

    #[test]
    fn test_move_left() {
        let mut piece = PhysicalPiece {
            x: 2,
            y: 0,
            rotation: 0,
            piece: BLOCK_I,
        };
        piece.move_left();
        assert_eq!(piece.x, 1);
        piece.move_left();
        assert_eq!(piece.x, 0);
    }

    #[test]
    fn test_newton() {
        let mut piece = PhysicalPiece {
            x: 0,
            y: 0,
            rotation: 0,
            piece: BLOCK_I,
        };
        piece.newton();
        assert_eq!(piece.y, 1);
        piece.newton();
        assert_eq!(piece.y, 2);
    }

    #[test]
    fn test_get_shape() {
        let piece = PhysicalPiece {
            x: 0,
            y: 0,
            rotation: 0,
            piece: BLOCK_I,
        };
        assert_eq!(piece.get_shape(), &[(0, 1), (1, 1), (2, 1), (3, 1)]);
        let piece = PhysicalPiece {
            x: 0,
            y: 0,
            rotation: 1,
            piece: BLOCK_I,
        };
        assert_eq!(piece.get_shape(), &[(2, 0), (2, 1), (2, 2), (2, 3)]);
    }
}
