use crate::common::direction::Direction;

// TODO: add tests

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Location {
    pub x: i32,
    pub y: i32,
}

impl Location {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// Return a new location moved in the given direction
    pub fn add(&self, dir: Direction) -> Self {
        Self {
            x: self.x + dir.dx(),
            y: self.y + dir.dy(),
        }
    }

    /// Get the cardinal direction from this location to another
    pub fn direction_to(&self, other: &Location) -> Direction {
        let dx = (other.x - self.x).signum();
        let dy = (other.y - self.y).signum();

        match (dx, dy) {
            (0, 1) => Direction::North,
            (1, 1) => Direction::Northeast,
            (1, 0) => Direction::East,
            (1, -1) => Direction::Southeast,
            (0, -1) => Direction::South,
            (-1, -1) => Direction::Southwest,
            (-1, 0) => Direction::West,
            (-1, 1) => Direction::Northwest,
            (0, 0) => Direction::Center,
            _ => Direction::Center,
        }
    }

    /// Calculate the squared Euclidean distance from this location to another
    pub fn distance_to(&self, other: &Location) -> i32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }

    /// Check if this Location is adjacent to another
    pub fn is_adjacent_to(&self, other: &Location) -> bool {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx.abs() <= 1 && dy.abs() <= 1
    }
}
