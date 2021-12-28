use super::backend::matrix::Matrix;
use super::cell::{cell::Cell, cell::Position};
use std::cmp;

use std::collections::BinaryHeap;

pub fn find_shortest<T: Cell>(
    matrix: &mut Matrix<T>,
    start: &Position,
    end: &Position,
) -> (Vec<T>, Matrix<T>) {
    let mut start_pos = matrix[start].clone();
    let mut pq = BinaryHeap::new();

    start_pos.set_visited(true);
    start_pos.set_distance(0);
    pq.push(start_pos);

    'outer: while !pq.is_empty() {
        let current = pq.pop().expect("Something went wrong");
        for each in matrix.get_neighbours(&current) {
            match each {
                Ok(neigbour) => {
                    let cost = current.get_distance()
                        + current.get_position().distance(&neigbour.get_position());
                    if !neigbour.is_wall()
                        && (!matrix[(neigbour.get_position())].get_visited()
                            || cost < matrix[(neigbour.get_position())].get_distance())
                    {
                        let heuristic = heuristic(neigbour.get_position(), &end);
                        matrix[(neigbour.get_position())].set_visited(true);
                        matrix[(neigbour.get_position())].set_distance(cost);
                        matrix[(neigbour.get_position())].set_heuristics(heuristic);
                        matrix[(neigbour.get_position())].set_parent(current.clone());
                        if neigbour.y() == end.y() && neigbour.x() == end.x() {
                            break 'outer;
                        }
                        pq.push(matrix[(neigbour.get_position())].clone());
                    }
                }
                Err(_) => {
                    continue;
                }
            }
        }
    }

    let path = get_parents(matrix, &matrix[start].clone(), end);

    for each in &path {
        matrix[each.get_position()].set_walk(true);
    }
    (path, matrix.clone())
}

pub fn heuristic(start: &Position, end: &Position) -> u32 {
    cmp::max(
        i32::abs(start.i32x() - end.i32x()),
        i32::abs(start.i32y() - end.i32y()),
    ) as u32
}

fn get_parents<T: Cell>(matrix: &mut Matrix<T>, start: &T, end: &Position) -> Vec<T> {
    let mut path: Vec<T> = Vec::new();
    let mut parent: (u32, u32) = (0, 0);
    if let Some(parent_pos) = *matrix[(end)].get_parent() {
        parent = parent_pos;
    }
    let mut last: T = matrix[parent].clone();
    while last.get_position() != start.get_position() {
        path.push(last.clone());
        if let Some(path) = last.get_parent() {
            last = matrix[*path].clone();
        }
    }
    path
}
