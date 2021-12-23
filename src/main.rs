mod backend;
mod cell;
mod djikstra;

use std::io;
use crate::backend::DataHandle;
use crate::cell::{
    MyCell,
};
use crate::cell::cell::Position;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();

    loop {
        buffer = String::new();
        stdin.read_line(&mut buffer);
        if !(buffer != String::from("quit") || buffer != String::from("q")) {
            let mut index: DataHandle<MyCell> = backend::get_data(String::from(buffer.trim()));
            let start = index.get_start();
            let end = index.get_end();
            assert!(start != Position { position: (0, 0) });
            assert!(end != Position { position: (0, 0) });
            let (path, matrix) = djikstra::find_shortest(&mut index.matrix(), &start, &end);
            index.write_image(&path, &matrix);
            println!("{}", path.len());
        } else {
            break;
        }
    }
    Ok(())
}
