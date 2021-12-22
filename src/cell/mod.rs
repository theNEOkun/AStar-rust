pub(crate) mod cell;

use std::cmp::{ Ordering };
use std::fmt::{self, Debug, Display, Formatter};
use cell::Cell;
use crate::cell::cell::Position;

#[derive(Clone)]
pub struct MyCell {
    position: Position,
    t: u8,
    distance: u32,
    heuristics: u32,
    parent: Option<(u32, u32)>,
    walk: bool,
}

impl Cell for MyCell {
    fn new(y: u32, x: u32, t: u8) -> MyCell {
        MyCell {
            position: Position { position: (y, x) },
            t,
            distance: 0,
            heuristics: 0,
            parent: None,
            walk: false,
        }
    }

    fn x(&self) -> u32 {
        self.position.x()
    }

    fn y(&self) -> u32 {
        self.position.y()
    }

    fn get_position(&self) -> &Position {
        &self.position
    }

    fn set_walk(&mut self, walk: bool) {
        self.walk = walk;
    }

    fn set_visited(&mut self, visited: bool) {
        self.t = if visited { 4 } else { 0 };
    }

    fn get_visited(&self) -> bool {
        self.t == 4
    }

    fn set_parent(&mut self, parent: MyCell) {
        self.parent = Some((parent.y(), parent.x()));
    }

    fn get_parent(&self) -> &Option<(u32, u32)> {
        &self.parent
    }

    fn set_distance(&mut self, distance: u32) {
        self.distance = distance
    }

    fn get_distance(&self) -> u32 {
        self.distance
    }

    fn set_heuristics(&mut self, heuristics: u32) {
        self.heuristics = heuristics
    }

    fn get_heuristics(&self) -> u32 {
        self.heuristics
    }

    fn get_combined(&self) -> u32 {
        self.heuristics + self.distance
    }

    fn get_type(&self) -> u8 {
        self.t
    }

    fn is_wall(&self) -> bool {
        self.t == 1
    }
}

impl Debug for MyCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(y: {} x: {}) ", self.y(), self.x())
    }
}

impl Display for MyCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, " {} ", if self.get_visited() {
            if self.walk {
                "5"
            } else {
                " "
            }
        } else {
            if self.is_wall() {
                "1"
            } else {
                "0"
            }
        })
    }
}

impl PartialOrd for MyCell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MyCell {
    fn cmp(&self, other: &Self) -> Ordering {
        other.get_combined().cmp(&self.get_combined())
            .then_with(|| self.distance.cmp(&other.distance))
    }
}

impl Eq for MyCell {}

impl PartialEq<Self> for MyCell {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}