use std::{fs::{
    self,
    File,
    ReadDir
}, io::{
    self,
    Write,
    Result,
}
};
use image::{
    io::Reader as ImageReader,
    GenericImage,
    GenericImageView,
    ImageBuffer,
    Pixel,
    Rgb,
    RgbImage
};
use crate::backend::matrix::Matrix;
use crate::cell::cell::Cell;

pub fn write_image(image: &RgbImage, file_name: &str) {
    image.save(format!("./resources/results/{}.png", file_name));
}

pub fn read_image(file_name: &str) -> RgbImage {
    let formatted = format!("./resources/images/{}.jpg", file_name);
    println!("{}", formatted);
    ImageReader::open(formatted)
        .expect("Something went wrong")
        .decode()
        .expect("Something else went wrong")
        .into_rgb8()
}

pub fn write_matrix<T: Cell>(matrix: &Matrix<T>) {
    let file = File::create("./resources/results/test");
    if let Ok(mut writer) = file {
        for y in 0..matrix.y_size() {
            for x in 0..matrix.x_size() {
                writer.write(format!("{}", &matrix[(y, x)]).as_ref());
                writer.write(" ".as_ref());
            }
            writer.write("\n".as_ref());
        }
    }
}

pub fn read_directory() -> io::Result<ReadDir> {
    fs::read_dir("./resources/images/")
}