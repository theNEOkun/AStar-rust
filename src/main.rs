mod backend;
mod cell;
mod djikstra;

use std::io;

use backend::file_handler::FileHandler;
use cell::{cell::Position, MyCell};

const RESULTS: &str = "./resources/results/";
const IMAGES: &str = "./resources/images/";

fn main() {
    if let Ok(_) = run() {};
}

fn run() -> io::Result<()> {
    let file_handler = FileHandler::new(String::from(RESULTS), String::from(IMAGES));
    let mut buffer;
    let stdin = io::stdin();
    if let Ok(values) = file_handler.read_directory() {
        for value in values {
            if let Some(name) = value?.path().file_name() {
                println!("{:?}", name);
            }
        }
    }

    println!("Choose a file");
    loop {
        buffer = String::new();
        if let Ok(_response) = stdin.read_line(&mut buffer) {
            if !(buffer == String::from("quit") || buffer == String::from("q")) {
                if let Ok(mut index) =
                    backend::get_data::<MyCell>(&file_handler, String::from(buffer.trim()))
                {
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
                    println!("No such file!");
                };
            } else {
                break;
            }
            println!("Choose a new file")
        };
    }
    Ok(())
}
