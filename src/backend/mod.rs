pub(crate) mod matrix;

use std::cmp;
use image::{GenericImage, GenericImageView, ImageBuffer, Pixel, Rgb, RgbImage};
use image::io::Reader as ImageReader;
use crate::backend::matrix::Matrix;
use crate::cell::cell::Cell;
use crate::Position;

pub struct DataHandle<T: Cell> {
    name: String,
    image: RgbImage,
    diff_x: u32,
    diff_y: u32,
    top_corner: Position,
    bottom_corner: Position,
    start: Position,
    end: Position,
    matrix: Matrix<T>,
}

impl<T: Cell> DataHandle<T> {
    pub fn matrix(&self) -> Matrix<T> {
        self.matrix.clone()
    }

    pub fn write_image(&mut self, path: &Vec<T>, matrix: &Matrix<T>) {
        for y_pos in self.top_corner.y()..self.bottom_corner.y() {
            for x_pos in self.top_corner.x()..self.bottom_corner.x() {
                if matrix[(y_pos - self.diff_y, x_pos - self.diff_x)].get_visited() {
                    self.image.put_pixel(
                        x_pos + (self.diff_x ),
                        y_pos + (self.diff_y ),
                        Rgb([0, 255, 0]));
                }
            }
        }
        for each in path {
            self.image.put_pixel(
                each.x() + (self.diff_x ),
                each.y() + (self.diff_y ),
                Rgb([255, 0, 0]));
        }

        self.image.save("./resources/results/output.png");
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
        if x_pos > self.top_corner.x() && y_pos > self.top_corner.y() && x_pos < self.bottom_corner.x() && y_pos < self.bottom_corner.y() {
            return Position { position: (y_pos - self.diff_y, x_pos - self.diff_x) }
        }
        if y_pos < self.top_corner.y() && x_pos > self.top_corner.x() {
            return Position {
                position: (0, self.search_x( 0, x_pos - self.diff_x))
            };
        }
        if y_pos > self.top_corner.y() && x_pos < self.top_corner.x() {
            return Position {
                position: (self.search_y(y_pos - self.diff_y, 0), 0)
            };
        }
        if y_pos > self.bottom_corner.y() && x_pos < self.bottom_corner.x() {

            return Position {
                position: ((self.matrix.y_size() - 1) as u32, self.search_x((self.matrix.y_size() - 1) as u32, x_pos - self.diff_x))
            };
        }
        if y_pos < self.bottom_corner.y() && x_pos > self.bottom_corner.x() {
            return Position {
                position: (self.search_y((y_pos - self.diff_y), (self.matrix.x_size() - 1) as u32), (self.matrix.x_size() - 1) as u32)
            };
        }

        Position { position: (0, 0) }
    }

    fn search_x(&self, y_pos: u32, x_pos: u32) -> u32 {
        let mut counter:u32 = 0;
        let mut start: bool = false;

        for i in 0..self.bottom_corner.x() {
            println!("({})", self.matrix[(y_pos, i)].is_wall());
            println!("({}, {})", y_pos, i);
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
        let mut counter:u32 = 0;
        let mut start: bool = false;

        for i in 0..self.bottom_corner.y() {
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

pub(crate) fn get_data<T: Cell>(file_name: String) -> DataHandle<T> {

    let image = get_image(&file_name);

    let top_corner = get_top_corner(&image);
    let bottom_corner = get_bottom_corner(&image);

    let diff_x = if top_corner.x() as i32 - 1 > 0 { top_corner.x() - 1 } else { top_corner.x() };
    let diff_y= if top_corner.y() as i32 - 1 > 0 { top_corner.y() - 1 } else { top_corner.y() };

    let smaller_image = image.view(
        top_corner.x(),
        top_corner.y(),
        (bottom_corner.x() - diff_x),
        (bottom_corner.y() - diff_y)
    ).to_image();

    DataHandle {
        name: file_name.clone(),
        image,
        diff_y,
        diff_x,
        top_corner,
        bottom_corner,
        start: Position { position: (0, 0) },
        end: Position { position: (0, 0) },
        matrix: get_matrix(&smaller_image),
    }
}

fn get_image(file_name: &str) -> RgbImage {
    ImageReader::open(format!("./resources/images/{}.jpg", file_name))
        .expect("Something went wrong")
        .decode()
        .expect("Something else went wrong")
        .into_rgb8()
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
        position: (topy, topx)
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
        position: (bottomy + 1, bottomx + 1)
    }
}

fn get_matrix<T:Cell>(image: &RgbImage) -> Matrix<T> {
    let new_size_x = image.width();
    let new_size_y = image.height();

    let mut matrix:Matrix<T> = Matrix::new(new_size_x as usize, new_size_y as usize);
    for y in 0..new_size_y {
        for x in 0..new_size_x {
            let pixel = image.get_pixel(x, y).channels();
            matrix[(y, x)] = T::new(y, x, test_colorus(pixel));
        }
    }
    matrix
}

fn test_colorus(pixel: &[u8]) -> u8 {
    if test_black(pixel) == true {
        return 1
    }
    if test_red(pixel) == true {
        return 2
    }
    if test_blue(pixel) == true {
        return 3
    }
    0
}

fn test_black(pixels: &[u8]) -> bool {
    if (pixels[0] == pixels[1] || pixels[0] == pixels[2]) && ( pixels[0] <= 160 || pixels[2] <= 160 )  {
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
    if pixels[0] >= 150 && (pixels[1] < 100 || pixels[2] < 100){
        return true;
    }
    false
}

fn test_blue(pixels: &[u8]) -> bool {
    if (pixels[0] < 100 || pixels[1] < 100) && pixels[2] >= 150 {
        return true;
    }
    false
}