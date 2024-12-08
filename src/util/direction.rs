use strum_macros::{Display, EnumIter};

#[derive(Clone, Copy, Debug, Display, EnumIter, Eq, Hash, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn is_opposite_of(&self, other: &Direction) -> bool {
        match self {
            Direction::North => *other == Direction::South,
            Direction::East => *other == Direction::West,
            Direction::South => *other == Direction::North,
            Direction::West => *other == Direction::East,
        }
    }

    pub fn get_opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    pub fn rotate_90_cwise(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn rotate_90_c_cwise(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

#[derive(Clone, Copy, Debug, Display, EnumIter, Eq, Hash, PartialEq)]
pub enum DirectionEx {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl DirectionEx {
    pub fn is_opposite_of(&self, other: &DirectionEx) -> bool {
        match self {
            DirectionEx::North => *other == DirectionEx::South,
            DirectionEx::NorthEast => *other == DirectionEx::SouthWest,
            DirectionEx::East => *other == DirectionEx::West,
            DirectionEx::SouthEast => *other == DirectionEx::NorthWest,
            DirectionEx::South => *other == DirectionEx::North,
            DirectionEx::SouthWest => *other == DirectionEx::NorthEast,
            DirectionEx::West => *other == DirectionEx::East,
            DirectionEx::NorthWest => *other == DirectionEx::SouthEast,
        }
    }

    pub fn get_opposite(&self) -> DirectionEx {
        match *self {
            DirectionEx::North => DirectionEx::South,
            DirectionEx::NorthEast => DirectionEx::SouthWest,
            DirectionEx::East => DirectionEx::West,
            DirectionEx::SouthEast => DirectionEx::NorthWest,
            DirectionEx::South => DirectionEx::North,
            DirectionEx::SouthWest => DirectionEx::NorthEast,
            DirectionEx::West => DirectionEx::East,
            DirectionEx::NorthWest => DirectionEx::SouthEast,
        }
    }

    pub fn rotate_90_cwise(&self) -> Self {
        match self {
            DirectionEx::North => DirectionEx::East,
            DirectionEx::NorthEast => DirectionEx::SouthEast,
            DirectionEx::East => DirectionEx::South,
            DirectionEx::SouthEast => DirectionEx::SouthWest,
            DirectionEx::South => DirectionEx::West,
            DirectionEx::SouthWest => DirectionEx::NorthWest,
            DirectionEx::West => DirectionEx::North,
            DirectionEx::NorthWest => DirectionEx::NorthEast,
        }
    }

    pub fn rotate_90_c_cwise(&self) -> Self {
        match self {
            DirectionEx::North => DirectionEx::West,
            DirectionEx::NorthEast => DirectionEx::NorthWest,
            DirectionEx::East => DirectionEx::North,
            DirectionEx::SouthEast => DirectionEx::NorthEast,
            DirectionEx::South => DirectionEx::East,
            DirectionEx::SouthWest => DirectionEx::SouthEast,
            DirectionEx::West => DirectionEx::South,
            DirectionEx::NorthWest => DirectionEx::SouthWest,
        }
    }
}
