mod backend;
mod cell;
mod djikstra;

use std::{
    io,
    fs,
};
use crate::{
    cell::{
        MyCell,
        cell::Position
    },
    backend::file_handler,
    backend::DataHandle
};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    if let Ok(Values) = file_handler::read_directory() {
        for value in Values {
            if let Some(name) = value?.path().file_name() {
                println!("{:?}", name);
            }
        }
    }

    println!("Choose a file");
    loop {
        buffer = String::new();
        stdin.read_line(&mut buffer);
        if !(buffer == String::from("quit") || buffer == String::from("q")) {
            let mut index: DataHandle<MyCell> = backend::get_data(String::from(buffer.trim()));
            let start = index.get_start();
            let end = index.get_end();
            assert!(start != Position { position: (0, 0) });
            assert!(end != Position { position: (0, 0) });
            println!("{}", start);
            println!("{}", end);
            let (path, matrix) = djikstra::find_shortest(&mut index.matrix(), &start, &end);
            index.write_image(&path, &matrix);
            println!("{}", path.len());
        } else {
            break;
        }
        println!("Choose a new file")
    }
    Ok(())
}
