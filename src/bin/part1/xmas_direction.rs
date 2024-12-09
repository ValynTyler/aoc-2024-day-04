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
    pub fn deltas(&self) -> Vec<(isize, isize)> {
        // XmasDirections::North => vec![
        //     ( 0,  0),
        //     ( 0,  1),
        //     ( 0,  2),
        //     ( 0,  3),
        // ],
        // XmasDirections::North_West => vec![
        //     (-0,  0),
        //     (-1,  1),
        //     (-2,  2),
        //     (-3,  3),
        // ],
        // XmasDirections::West => vec![
        //     (-0,  0),
        //     (-1,  0),
        //     (-2,  0),
        //     (-3,  0),
        // ],
        // XmasDirections::South_West => vec![
        //     (-0, -0),
        //     (-1, -1),
        //     (-2, -2),
        //     (-3, -3),
        // ],
        // XmasDirections::South => vec![
        //     ( 0, -0),
        //     ( 0, -1),
        //     ( 0, -2),
        //     ( 0, -3),
        // ],
        // XmasDirections::South_East => vec![
        //     ( 0,  0),
        //     (-1,  1),
        //     (-2,  2),
        //     (-3,  3),
        // ],
        // XmasDirections::East => vec![
        //     ( 0,  0),
        //     ( 0,  1),
        //     ( 0,  2),
        //     ( 0,  3),
        // ],
        // XmasDirections::North_East => vec![
        //     ( 0,  0),
        //     ( 1,  1),
        //     ( 2,  2),
        //     ( 3,  3),
        // ],
        todo!()
    }
}
