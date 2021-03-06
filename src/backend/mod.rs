pub mod file_handler;
pub(crate) mod matrix;

use image::{GenericImageView, Pixel, Rgb, RgbImage};
use std::{
    cmp,
    io::{self},
};

use self::file_handler::FileHandler;

use crate::{backend::matrix::Matrix, cell::cell::Cell, cell::cell::Position};

pub struct DataHandle<'a, T: Cell> {
    name: String,
    image: RgbImage,
    diff_x: u32,
    diff_y: u32,
    top_corner: Position,
    bottom_corner: Position,
    start: Position,
    end: Position,
    matrix: Matrix<T>,
    file_handler: &'a FileHandler,
}

impl<'a, T: Cell> DataHandle<'a, T> {
    pub fn matrix(&self) -> Matrix<T> {
        self.matrix.clone()
    }

    pub fn write_image(&mut self, path: &Vec<T>, matrix: &Matrix<T>) {
        for y_pos in self.top_corner.y()..(self.bottom_corner.y() - 1) {
            for x_pos in self.top_corner.x()..(self.bottom_corner.x() - 1) {
                if matrix[(y_pos - (self.diff_y), x_pos - (self.diff_x))].get_visited() {
                    self.image.put_pixel(x_pos, y_pos, Rgb([0, 255, 0]));
                }
            }
        }
        for each in path {
            self.image.put_pixel(
                each.x() + (self.diff_x),
                each.y() + (self.diff_y),
                Rgb([255, 0, 0]),
            );
        }
        self.file_handler.write_image(&self.image, "output")
    }

    pub fn get_start(&self) -> Position {
        for y_pos in 0..self.image.height() {
            for x_pos in 0..self.image.width() {
                if test_red(self.image.get_pixel(x_pos, y_pos).channels()) {
                    return self.create_position(x_pos, y_pos);
                }
            }
        }
        Position { position: (0, 0) }
    }

    pub fn get_end(&self) -> Position {
        for y_pos in 0..self.image.height() {
            for x_pos in 0..self.image.width() {
                if test_blue(self.image.get_pixel(x_pos, y_pos).channels()) {
                    return self.create_position(x_pos, y_pos);
                }
            }
        }
        Position { position: (0, 0) }
    }

    fn create_position(&self, x_pos: u32, y_pos: u32) -> Position {
        if x_pos > self.top_corner.x()
            && y_pos > self.top_corner.y()
            && x_pos < self.bottom_corner.x()
            && y_pos < self.bottom_corner.y()
        {
            return Position {
                position: (y_pos - self.diff_y, x_pos - self.diff_x),
            };
        }
        if y_pos < self.top_corner.y() && x_pos > self.top_corner.x() {
            return Position {
                position: (0, self.search_x(0, x_pos - self.diff_x)),
            };
        }
        if y_pos > self.top_corner.y() && x_pos < self.top_corner.x() {
            return Position {
                position: (self.search_y(y_pos - self.diff_y, 0), 0),
            };
        }
        if y_pos > self.bottom_corner.y() && x_pos < self.bottom_corner.x() {
            return Position {
                position: (
                    (self.matrix.y_size() - 1) as u32,
                    self.search_x((self.matrix.y_size() - 1) as u32, x_pos - self.diff_x),
                ),
            };
        }
        if y_pos < self.bottom_corner.y() && x_pos > self.bottom_corner.x() {
            return Position {
                position: (
                    self.search_y(y_pos - self.diff_y, (self.matrix.x_size() - 1) as u32),
                    (self.matrix.x_size() - 1) as u32,
                ),
            };
        }

        Position { position: (0, 0) }
    }

    fn search_x(&self, y_pos: u32, x_pos: u32) -> u32 {
        let mut counter: u32 = 0;
        let mut start: bool = false;

        for i in cmp::max(x_pos, self.top_corner.x())..(self.matrix.x_size() as u32) {
            if self.matrix[(y_pos, i)].is_wall() && !start {
                start = true;
            }
            if !self.matrix[(y_pos, i)].is_wall() && start {
                counter += 1;
            }
            if counter >= 5 {
                return i;
            }
        }
        counter = 0;
        for i in (0..cmp::min(x_pos, self.matrix.x_size() as u32)).rev() {
            if self.matrix[(y_pos, i)].is_wall() && !start {
                start = true;
            }
            if !self.matrix[(y_pos, i)].is_wall() && start {
                counter += 1;
            }
            if counter >= 5 {
                return i;
            }
        }
        1
    }

    fn search_y(&self, y_pos: u32, x_pos: u32) -> u32 {
        let mut counter: u32 = 0;
        let mut start: bool = false;

        for i in cmp::max(y_pos, self.top_corner.y())..(self.matrix.y_size() as u32) {
            if self.matrix[(i, x_pos)].is_wall() && !start {
                start = true;
            }
            if !self.matrix[(i, x_pos)].is_wall() && start {
                counter += 1;
            }
            if counter >= 5 {
                return i;
            }
        }
        counter = 0;
        for i in (0..cmp::min(y_pos, self.matrix.y_size() as u32)).rev() {
            if self.matrix[(i, x_pos)].is_wall() && !start {
                start = true;
            }
            if !self.matrix[(i, x_pos)].is_wall() && start {
                counter += 1;
            }
            if counter >= 5 {
                return i;
            }
        }
        1
    }
}

pub(crate) fn get_data<T: Cell>(
    file_handler: &FileHandler,
    file_name: String,
) -> Result<DataHandle<T>, io::Error> {
    match file_handler.read_image(&file_name) {
        Ok(result_image) => {
            let image = result_image.into_rgb8();
            let top_corner = get_top_corner(&image);
            let bottom_corner = get_bottom_corner(&image);

            let diff_x = top_corner.x();
            let diff_y = top_corner.y();

            let smaller_image = image
                .view(
                    top_corner.x(),
                    top_corner.y(),
                    bottom_corner.x() - diff_x,
                    bottom_corner.y() - diff_y,
                )
                .to_image();

            return Ok(DataHandle {
                name: file_name.clone(),
                image,
                diff_y,
                diff_x,
                top_corner,
                bottom_corner,
                start: Position { position: (0, 0) },
                end: Position { position: (0, 0) },
                matrix: get_matrix(&smaller_image),
                file_handler,
            });
        }
        Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Wrong input")),
    }
}

fn get_top_corner(image: &RgbImage) -> Position {
    let full_size_y = image.height();
    let full_size_x = image.width();

    let mut topx = 0;
    let mut topy = 0;
    'outerY: for y_pos in 0..full_size_y {
        for x_pos in 0..full_size_x {
            if test_black(image.get_pixel(x_pos, y_pos).channels()) {
                topy = y_pos;
                break 'outerY;
            }
        }
    }
    'outerX: for x_pos in 0..full_size_x {
        for y_pos in 0..full_size_y {
            if test_black(image.get_pixel(x_pos, y_pos).channels()) {
                topx = x_pos;
                break 'outerX;
            }
        }
    }
    Position {
        position: (topy, topx),
    }
}

fn get_bottom_corner(image: &RgbImage) -> Position {
    let full_size_y = image.height();
    let full_size_x = image.width();

    let mut bottomx = 0;
    let mut bottomy = 0;
    'outerY: for y_pos in (0..full_size_y).rev() {
        for x_pos in (0..full_size_x).rev() {
            if test_black(image.get_pixel(x_pos, y_pos).channels()) {
                bottomy = y_pos;
                break 'outerY;
            }
        }
    }
    'outerX: for x_pos in (0..full_size_x).rev() {
        for y_pos in (0..full_size_y).rev() {
            if test_black(image.get_pixel(x_pos, y_pos).channels()) {
                bottomx = x_pos;
                break 'outerX;
            }
        }
    }
    Position {
        position: (bottomy + 1, bottomx + 1),
    }
}

fn get_matrix<T: Cell>(image: &RgbImage) -> Matrix<T> {
    let new_size_x = image.width();
    let new_size_y = image.height();

    let mut matrix: Matrix<T> = Matrix::new(new_size_x as usize, new_size_y as usize);
    for y in 0..new_size_y {
        for x in 0..new_size_x {
            let pixel = image.get_pixel(x, y).channels();
            matrix[(y, x)] = T::new(
                y,
                x,
                if test_adjecent(image, x, y) {
                    1
                } else {
                    test_colorus(pixel)
                },
            );
        }
    }
    matrix
}

fn test_colorus(pixel: &[u8]) -> u8 {
    if test_black(pixel) {
        return 1;
    }
    if test_red(pixel) {
        return 2;
    }
    if test_blue(pixel) {
        return 3;
    }
    0
}

fn test_adjecent(image: &RgbImage, x_pos: u32, y_pos: u32) -> bool {
    if (x_pos as i32) - 1 < 0
        || (y_pos as i32) - 1 < 0
        || x_pos + 1 >= image.width()
        || y_pos + 1 >= image.height()
    {
        return false;
    }
    if test_black(image.get_pixel(x_pos, y_pos + 1).channels())
        && test_black(image.get_pixel(x_pos + 1, y_pos).channels())
    {
        return true;
    }
    if test_black(image.get_pixel(x_pos, y_pos - 1).channels())
        && test_black(image.get_pixel(x_pos - 1, y_pos).channels())
    {
        return true;
    }
    if test_black(image.get_pixel(x_pos, y_pos + 1).channels())
        && test_black(image.get_pixel(x_pos - 1, y_pos).channels())
    {
        return true;
    }
    if test_black(image.get_pixel(x_pos, y_pos - 1).channels())
        && test_black(image.get_pixel(x_pos + 1, y_pos).channels())
    {
        return true;
    }
    false
}

fn test_black(pixels: &[u8]) -> bool {
    if (pixels[0] == pixels[1] || pixels[0] == pixels[2]) && (pixels[0] <= 160 || pixels[2] <= 160)
    {
        return true;
    }
    if pixels[0] < 50 && pixels[1] < pixels[0] && pixels[2] < pixels[0] {
        return true;
    }
    if pixels[1] < 50 && pixels[0] < pixels[1] && pixels[2] < pixels[1] {
        return true;
    }
    if pixels[2] < 50 && pixels[1] < pixels[2] && pixels[0] < pixels[2] {
        return true;
    }
    false
}

fn test_red(pixels: &[u8]) -> bool {
    if pixels[0] >= 150 && (pixels[1] < 100 || pixels[2] < 100) {
        true
    } else {
        false
    }
}

fn test_blue(pixels: &[u8]) -> bool {
    if (pixels[0] < 100 || pixels[1] < 100) && pixels[2] >= 150 {
        true
    } else {
        false
    }
}
