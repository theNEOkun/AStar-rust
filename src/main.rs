mod backend;
mod cell;
mod djikstra;

use crate::backend::DataHandle;
use crate::cell::{
    MyCell,
};
use crate::cell::cell::Position;

fn main() {
    let file_name = "squareeasy1";
    let mut index: DataHandle<MyCell> = backend::get_data(String::from(file_name));
    let mut start = Position{ position: (518, 0) };
    let end = Position { position: (640, 59) };
    let path = djikstra::find_shortest(index.matrix(), start, end);
    index.write_image(path);
}
