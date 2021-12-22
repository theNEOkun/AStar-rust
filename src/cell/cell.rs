use std::cmp;
use std::fmt::{self, Display, Debug, Formatter};

#[derive(Clone)]
pub struct Position {
    pub(crate) position: (u32, u32)
}

impl Position {
    pub fn x(&self) -> u32 {
        self.position.1
    }

    pub fn y(&self) -> u32 {
        self.position.0
    }

    pub fn i32x(&self) -> i32 {
        self.position.1 as i32
    }

    pub fn i32y(&self) -> i32 {
        self.position.0 as i32
    }

    pub fn distance(&self, other: &Self) -> u32 {
        cmp::max(i32::abs(self.i32x() - other.i32x()), i32::abs(self.i32y() - other.i32y())) as u32
    }
}

impl PartialEq<Self> for Position {
    fn eq(&self, other: &Self) -> bool {
        (self.x() == other.x()) && (self.y() == other.y())
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(y: {} x: {}) ", self.position.0, self.position.1)
    }
}

pub trait Cell: Display + Debug + Eq + PartialEq<Self> + PartialOrd<Self> + Ord + Clone {
    fn new(x: u32, y: u32, t: u8) -> Self;

    fn x(&self) -> u32;

    fn y(&self) -> u32;

    fn get_position(&self) -> &Position;

    fn set_walk(&mut self, walk: bool);

    fn set_visited(&mut self, visited: bool);

    fn get_visited(&self) -> bool;

    fn set_parent(&mut self, parent: Self);

    fn get_parent(&self) -> &Option<(u32, u32)>;

    fn set_distance(&mut self, distance: u32);

    fn get_distance(&self) -> u32;

    fn set_heuristics(&mut self, heuristics: u32);

    fn get_heuristics(&self) -> u32;

    fn get_combined(&self) -> u32;

    fn get_type(&self) -> u8;

    fn is_wall(&self) -> bool;
}