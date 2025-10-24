#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
    Center,
}

impl Direction {
    /// Returns the change in the x direction
    pub fn dx(&self) -> i32 {
        use Direction::*;
        match self {
            North => 0,
            Northeast => 1,
            East => 1,
            Southeast => 1,
            South => 0,
            Southwest => -1,
            West => -1,
            Northwest => -1,
            Center => 0,
        }
    }

    /// Returns the change in the y direction
    pub fn dy(&self) -> i32 {
        use Direction::*;
        match self {
            North => 1,
            Northeast => 1,
            East => 0,
            Southeast => -1,
            South => -1,
            Southwest => -1,
            West => 0,
            Northwest => 1,
            Center => 0,
        }
    }

    /// Rotate 45 degrees counter-clockwise
    pub fn rotate_left(&self) -> Self {
        use Direction::*;
        match self {
            Center => Center,
            North => Northwest,
            Northwest => West,
            West => Southwest,
            Southwest => South,
            South => Southeast,
            Southeast => East,
            East => Northeast,
            Northeast => North,
        }
    }

    /// Rotate 45 degrees clockwise
    pub fn rotate_right(&self) -> Self {
        use Direction::*;
        match self {
            Center => Center,
            North => Northeast,
            Northeast => East,
            East => Southeast,
            Southeast => South,
            South => Southwest,
            Southwest => West,
            West => Northwest,
            Northwest => North,
        }
    }

    /// Get the opposite direction (180 degrees)
    pub fn opposite(&self) -> Self {
        use Direction::*;
        match self {
            Center => Center,
            North => South,
            Northeast => Southwest,
            East => West,
            Southeast => Northwest,
            South => North,
            Southwest => Northeast,
            West => East,
            Northwest => Southeast,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dx_dy() {
        assert_eq!(Direction::North.dx(), 0);
        assert_eq!(Direction::North.dy(), 1);
        assert_eq!(Direction::Northeast.dx(), 1);
        assert_eq!(Direction::Northeast.dy(), 1);
        assert_eq!(Direction::Northwest.dx(), -1);
        assert_eq!(Direction::Northwest.dy(), 1);
        assert_eq!(Direction::South.dx(), 0);
        assert_eq!(Direction::South.dy(), -1);
        assert_eq!(Direction::Southeast.dx(), 1);
        assert_eq!(Direction::Southeast.dy(), -1);
        assert_eq!(Direction::Southwest.dx(), -1);
        assert_eq!(Direction::Southwest.dy(), -1);
        assert_eq!(Direction::East.dx(), 1);
        assert_eq!(Direction::East.dy(), 0);
        assert_eq!(Direction::West.dx(), -1);
        assert_eq!(Direction::West.dy(), 0);
        assert_eq!(Direction::Center.dx(), 0);
        assert_eq!(Direction::Center.dy(), 0);
    }

    #[test]
    fn test_rotate_left() {
        assert_eq!(Direction::North.rotate_left(), Direction::Northwest);
        assert_eq!(Direction::Northwest.rotate_left(), Direction::West);
        assert_eq!(Direction::West.rotate_left(), Direction::Southwest);
        assert_eq!(Direction::Southwest.rotate_left(), Direction::South);
        assert_eq!(Direction::South.rotate_left(), Direction::Southeast);
        assert_eq!(Direction::Southeast.rotate_left(), Direction::East);
        assert_eq!(Direction::East.rotate_left(), Direction::Northeast);
        assert_eq!(Direction::Northeast.rotate_left(), Direction::North);
        assert_eq!(Direction::Center.rotate_left(), Direction::Center);
    }

    #[test]
    fn test_rotate_right() {
        assert_eq!(Direction::North.rotate_right(), Direction::Northeast);
        assert_eq!(Direction::Northeast.rotate_right(), Direction::East);
        assert_eq!(Direction::East.rotate_right(), Direction::Southeast);
        assert_eq!(Direction::Southeast.rotate_right(), Direction::South);
        assert_eq!(Direction::South.rotate_right(), Direction::Southwest);
        assert_eq!(Direction::Southwest.rotate_right(), Direction::West);
        assert_eq!(Direction::West.rotate_right(), Direction::Northwest);
        assert_eq!(Direction::Northwest.rotate_right(), Direction::North);
        assert_eq!(Direction::Center.rotate_right(), Direction::Center);
    }

    #[test]
    fn test_opposite() {
        assert_eq!(Direction::North.opposite(), Direction::South);
        assert_eq!(Direction::Northeast.opposite(), Direction::Southwest);
        assert_eq!(Direction::East.opposite(), Direction::West);
        assert_eq!(Direction::Southeast.opposite(), Direction::Northwest);
        assert_eq!(Direction::South.opposite(), Direction::North);
        assert_eq!(Direction::Southwest.opposite(), Direction::Northeast);
        assert_eq!(Direction::West.opposite(), Direction::East);
        assert_eq!(Direction::Northwest.opposite(), Direction::Southeast);
        assert_eq!(Direction::Center.opposite(), Direction::Center);
    }
}
