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
    let start = index.get_start();
    let end = index.get_end();
    assert!(start != Position{ position: (0, 0)});
    assert!(end != Position{ position: (0, 0)});
    println!("{}", start);
    println!("{}", end);
    let (path, matrix) = djikstra::find_shortest(&mut index.matrix(), &start, &end);
    index.write_image(&path, &matrix);
}
