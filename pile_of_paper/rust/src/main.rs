// Thomas Ring
// May 16, 2015
// Input a list of directions for a canvas with colored papers and calculates the area of each color
// main.rs

use std::fs::File;
use std::io::Read;
use std::path::Path;

extern crate pile_of_paper;
use pile_of_paper::canvas::{Canvas};

fn main() {
    let example_input = "20 10\n1 5 5 10 3\n2 0 0 7 7\n".to_string();
    let example_canvas: Canvas = Canvas::new_from_string(example_input);
    println!("Area of colors: {}", example_canvas.area_of_colors());

    for file in test_files() {
        if !test_canvas_with_file(file.to_string()) {
            println!("Error reading {}", file);
        }
    }
}

fn test_canvas_with_file(filename: String) -> bool {
    println!("Testing {}.", filename);

    let mut input_file = file_for_string(format!("{}.in", filename));
    let mut output_file = file_for_string(format!("{}.out", filename));

    let mut input_string = String::new();
    let input_length = input_file.read_to_string(&mut input_string);

    let canvas = Canvas::new_from_string(input_string);
    let area_s = canvas.area_of_colors();

    println!("Area of colors: {}", area_s);

    let mut output_string = String::new();
    let output_length = output_file.read_to_string(&mut output_string);

    *output_string == area_s
}

fn file_for_string(filename: String) -> File {
    match File::open(Path::new(&filename)) {
        Ok(file) => {
            file
        },
        Err(err) => panic!("Error reading file: {}", err),
    }
}

fn test_files() -> Vec<&'static str> {
    vec!["100rects100x100", "1Krects100x100", "100rects10Kx10K", "100rects100Kx100K"]
}
