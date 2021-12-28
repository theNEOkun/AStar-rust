use std::{fs::{
    self,
    File,
    ReadDir,
}, io::{
    self,
    Write,
    ErrorKind
}
};
use image::{
    io::Reader as ImageReader,
    RgbImage,
    DynamicImage, ImageError,
};
use crate::backend::matrix::Matrix;
use crate::cell::cell::Cell;

#[derive(Clone)]
pub struct FileHandler {
    results: String,
    images: String,
}

impl FileHandler {
    pub fn new(results: String, images: String) -> FileHandler {
        FileHandler {
            results,
            images,
        }
    }

    pub fn write_image(&self, image: &RgbImage, file_name: &str) {
        if let Ok(_resp) = image.save(format!("{}{}.png", self.results, file_name)) {
            println!("File saved");
        };
    }

    pub fn read_image(&self, file_name: &str) -> Result<DynamicImage, ImageError> {
        if let Ok(value) = ImageReader::open(format!("{}{}.jpg", self.images, file_name)) {
            return value.decode();
        };
        Err(ImageError::IoError(io::Error::new(ErrorKind::InvalidInput, "Well shit")))
    }

    pub fn write_matrix<T: Cell>(&self, matrix: &Matrix<T>) {
        if let Ok(mut writer) = File::create(format!("{}test", self.results)) {
            for y in 0..matrix.y_size() {
                for x in 0..matrix.x_size() {
                    writer.write(format!("{}", &matrix[(y, x)]).as_ref());
                    writer.write(" ".as_ref());
                }
                writer.write("\n".as_ref());
            }
        }
    }

    pub fn read_directory(&self) -> io::Result<ReadDir> {
        fs::read_dir(&self.images)
    }
}