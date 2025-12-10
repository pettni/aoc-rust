use std::ops::Neg;

// 2D direction type
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Ord, PartialOrd)]
pub enum Dir {
    N = 0,
    E = 1,
    S = 2,
    W = 3,
    NW = 4,
    NE = 5,
    SW = 6,
    SE = 7,
}

pub const DIRECTIONS: [Dir; 4] = [Dir::N, Dir::E, Dir::S, Dir::W];

pub const DIRECTIONS8: [Dir; 8] = [
    Dir::N,
    Dir::NE,
    Dir::E,
    Dir::SE,
    Dir::S,
    Dir::SW,
    Dir::W,
    Dir::NW,
];

impl Dir {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '^' => Some(Dir::N),
            '>' => Some(Dir::E),
            'v' => Some(Dir::S),
            '<' => Some(Dir::W),
            _ => None,
        }
    }

    pub fn turn_right(&self) -> Dir {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
            Dir::NE => Dir::SE,
            Dir::SE => Dir::SW,
            Dir::SW => Dir::NW,
            Dir::NW => Dir::NE,
        }
    }

    pub fn turn_left(&self) -> Dir {
        match self {
            Dir::N => Dir::W,
            Dir::E => Dir::N,
            Dir::S => Dir::E,
            Dir::W => Dir::S,
            Dir::NE => Dir::NW,
            Dir::SE => Dir::NE,
            Dir::SW => Dir::SE,
            Dir::NW => Dir::SW,
        }
    }

    pub fn turn_around(&self) -> Dir {
        match self {
            Dir::N => Dir::S,
            Dir::E => Dir::W,
            Dir::S => Dir::N,
            Dir::W => Dir::E,
            Dir::NE => Dir::SW,
            Dir::SE => Dir::NW,
            Dir::SW => Dir::NE,
            Dir::NW => Dir::SE,
        }
    }
}

impl Neg for Dir {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.turn_around()
    }
}
