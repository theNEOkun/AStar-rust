use crate::cell::cell::{Cell, Position};
use std::ops::{Index, IndexMut};

type Result<T> = std::result::Result<T, DoubleError>;

pub struct DoubleError;

pub struct MatrixIterator<T: Cell> {
    matrix: Vec<Vec<T>>,
    x_pos: usize,
    y_pos: usize,
}

impl<T: Cell> MatrixIterator<T> {
    fn new(matrix: Vec<Vec<T>>) -> MatrixIterator<T> {
        MatrixIterator {
            matrix,
            x_pos: 0,
            y_pos: 0,
        }
    }
}

impl<T: Cell> Iterator for MatrixIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = self.matrix[self.y_pos][self.x_pos].clone();
        self.x_pos += 1;
        if self.x_pos as usize == self.matrix[0].len() {
            self.y_pos += 1;
        }
        if self.x_pos as usize > self.matrix[0].len() && self.y_pos > self.matrix.len() {
            return None
        }
        Some(curr)
    }
}

#[derive(Clone)]
pub struct Matrix<T: Cell> {
    matrix: Vec<Vec<T>>,
    y_size: usize,
    x_size: usize,
}

impl<T: Cell> Matrix<T> {
    pub fn new(x_size: usize, y_size: usize) -> Matrix<T> {
        Matrix {
            matrix: vec![vec![T::new(0, 0, 0); x_size]; y_size],
            y_size,
            x_size,
        }
    }

    pub fn y_size(&self) -> usize {
        self.y_size
    }

    pub fn x_size(&self) -> usize {
        self.x_size
    }

    pub fn get_neighbours(&self, position: &T) -> Vec<Result<T>> {
        let mut arr: Vec<Result<T>> = Vec::new();

        let test_y = position.y() as i32;
        let test_x = position.x() as i32;

        let pos_y = position.y();
        let pos_x = position.x();
        if test_x - 1 > 0 && (((test_y + 1) as usize) < self.y_size) {
            arr.push(Ok(self[(pos_y + 1, pos_x - 1)].clone()));
        }
        if ((test_y + 1) as usize) < self.y_size {
            arr.push(Ok(self[(pos_y + 1, pos_x)].clone()));
        }
        if ((test_y + 1) as usize) < self.y_size && ((test_x + 1) as usize) < self.x_size {
            arr.push(Ok(self[(pos_y + 1, pos_x + 1)].clone()));
        }
        if ((test_x - 1) as usize) < self.x_size {
            arr.push(Ok(self[(pos_y, pos_x - 1)].clone()));
        }
        if ((test_x + 1) as usize) < self.x_size {
            arr.push(Ok(self[(pos_y, pos_x + 1)].clone()));
        }
        if test_x - 1 > 0 && test_y - 1 > 0 {
            arr.push(Ok(self[(pos_y - 1, pos_x - 1)].clone()));
        }
        if test_y - 1 > 0 {
            arr.push(Ok(self[(pos_y - 1, pos_x)].clone()));
        }
        if test_y - 1 > 0 && (((test_x + 1) as usize) < self.x_size) {
            arr.push(Ok(self[(pos_y - 1, pos_x + 1)].clone()));
        }

        arr
    }

    pub fn show(&self) {
        for index_y in 0..self.y_size {
            for index_x in 0..self.x_size {
                print!("{}", self[(index_y, index_x)])
            }
            println!()
        }
    }
}

impl<T: Cell> IntoIterator for Matrix<T> {
    type Item = T;
    type IntoIter = MatrixIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        MatrixIterator::new(self.matrix)
    }
}

impl<T: Cell> Index<(u32, u32)> for Matrix<T> {
    type Output = T;

    fn index(&self, (y, x): (u32, u32)) -> &Self::Output {
        let y_pos = y as usize;
        let x_pos = x as usize;
        assert!(y_pos < self.y_size || x_pos < self.x_size );
        &self.matrix[y_pos][x_pos]
    }
}

impl<T: Cell> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (y, x): (usize, usize)) -> &Self::Output {
        assert!(y < self.y_size || x < self.x_size );
        &self.matrix[y][x]
    }
}

impl<T: Cell> IndexMut<(u32, u32)> for Matrix<T> {
    fn index_mut(&mut self, (y, x): (u32, u32)) -> &mut Self::Output {
        let y_pos = y as usize;
        let x_pos = x as usize;
        assert!(y_pos < self.y_size || x_pos < self.x_size );
        &mut self.matrix[y_pos][x_pos]
    }
}

impl<T: Cell> Index<&Position> for Matrix<T> {
    type Output = T;

    fn index(&self, pos: &Position) -> &Self::Output {
        let y_pos = pos.y() as usize;
        let x_pos = pos.x() as usize;
        assert!(y_pos < self.y_size || x_pos < self.x_size );
        &self.matrix[y_pos][x_pos]
    }
}

impl<T: Cell> IndexMut<&Position> for Matrix<T> {
    fn index_mut(&mut self, pos: &Position) -> &mut Self::Output {
        let y_pos = pos.y() as usize;
        let x_pos = pos.x() as usize;
        assert!(y_pos < self.y_size || x_pos < self.x_size );
        &mut self.matrix[y_pos][x_pos]
    }
}
