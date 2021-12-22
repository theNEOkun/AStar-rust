mod backend;
mod cell;
mod djikstra;

use crate::backend::DataHandle;
use crate::cell::{
    MyCell,
};
use crate::cell::cell::Position;

fn main() {
    let file_name = "testImage";
    let mut index: DataHandle<MyCell> = backend::get_data(String::from(file_name));
    let mut start = Position{ position: (18, 2) };
    let end = Position { position: (2, 18) };
    let (path, matrix) = djikstra::find_shortest(&mut index.matrix(), start, end);
    index.matrix().show();
    index.write_image(&path, &matrix);
}
