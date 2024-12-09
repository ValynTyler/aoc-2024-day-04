pub enum XmasDirection {
    North,
    NorthWest,
    West,
    SouthWest,
    South,
    SouthEast,
    East,
    NorthEast,
}

impl XmasDirection {
    pub fn deltas(&self) -> Vec<(isize, isize)> {
        match self {
            XmasDirection::North => vec![
                ( 0,  0),
                ( 0,  1),
                ( 0,  2),
                ( 0,  3),
            ],
            XmasDirection::NorthWest => vec![
                (-0,  0),
                (-1,  1),
                (-2,  2),
                (-3,  3),
            ],
            XmasDirection::West => vec![
                (-0,  0),
                (-1,  0),
                (-2,  0),
                (-3,  0),
            ],
            XmasDirection::SouthWest => vec![
                (-0, -0),
                (-1, -1),
                (-2, -2),
                (-3, -3),
            ],
            XmasDirection::South => vec![
                ( 0, -0),
                ( 0, -1),
                ( 0, -2),
                ( 0, -3),
            ],
            XmasDirection::SouthEast => vec![
                ( 0,  0),
                (-1,  1),
                (-2,  2),
                (-3,  3),
            ],
            XmasDirection::East => vec![
                ( 0,  0),
                ( 0,  1),
                ( 0,  2),
                ( 0,  3),
            ],
            XmasDirection::NorthEast => vec![
                ( 0,  0),
                ( 1,  1),
                ( 2,  2),
                ( 3,  3),
            ],
        }
    }
}
