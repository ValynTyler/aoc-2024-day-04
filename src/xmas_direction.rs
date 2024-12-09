pub struct XmasDirections {
    north: bool,
    north_west: bool,
    west: bool,
    south_west: bool,
    south: bool,
    south_east: bool,
    east: bool,
    north_east: bool,
}

impl XmasDirections {
    pub const ALL: Self = Self {
        north: true,
        north_west: true,
        west: true,
        south_west: true,
        south: true,
        south_east: true,
        east: true,
        north_east: true,
    };

    pub const NONE: Self = Self {
        north: false,
        north_west: false,
        west: false,
        south_west: false,
        south: false,
        south_east: false,
        east: false,
        north_east: false,
    };

    pub fn add(&mut self, dir: XmasDirection) {
        match dir {
            XmasDirection::North => self.north = true,
            XmasDirection::South => self.south = true,
            XmasDirection::East => self.east = true,
            XmasDirection::West => self.west = true,
            XmasDirection::NorthEast => self.north_east = true,
            XmasDirection::NorthWest => self.north_west = true,
            XmasDirection::SouthEast => self.south_east = true,
            XmasDirection::SouthWest => self.south_west = true,
        }
    }

    pub fn directions(&self) -> Vec<XmasDirection> {
        let mut v = vec![];

        if self.north == true { v.push(XmasDirection::North) }
        if self.south == true { v.push(XmasDirection::South) }
        if self.east == true { v.push(XmasDirection::East) }
        if self.west == true { v.push(XmasDirection::West) }
        if self.north_east == true { v.push(XmasDirection::NorthEast) }
        if self.north_west == true { v.push(XmasDirection::NorthWest) }
        if self.south_east == true { v.push(XmasDirection::SouthEast) }
        if self.south_west == true { v.push(XmasDirection::SouthWest) }

        v
    }
}

pub enum XmasDirection {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl XmasDirection {
    pub fn deltas(&self) -> Vec<(isize, isize)> {
        match self {
            XmasDirection::North => vec![
                ( 0, -1),
                ( 0, -2),
                ( 0, -3),
            ],
            XmasDirection::NorthWest => vec![
                (-1, -1),
                (-2, -2),
                (-3, -3),
            ],
            XmasDirection::West => vec![
                (-1,  0),
                (-2,  0),
                (-3,  0),
            ],
            XmasDirection::SouthWest => vec![
                (-1,  1),
                (-2,  2),
                (-3,  3),
            ],
            XmasDirection::South => vec![
                ( 0,  1),
                ( 0,  2),
                ( 0,  3),
            ],
            XmasDirection::SouthEast => vec![
                (-1, -1),
                (-2, -2),
                (-3, -3),
            ],
            XmasDirection::East => vec![
                ( 1,  0),
                ( 2,  0),
                ( 3,  0),
            ],
            XmasDirection::NorthEast => vec![
                ( 1, -1),
                ( 2, -2),
                ( 3, -3),
            ],
        }
    }
}
