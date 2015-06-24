// Thomas Ring
// May 16, 2015
// Input a list of directions for a canvas with colored papers and calculates the area of each color
// lib.rs

#![crate_type = "lib"]
#![crate_name = "pile_of_paper"]

#![feature(core)]
extern crate core;

pub mod canvas {
    use core::str::FromStr;

    // The whole canvas
    pub struct Canvas {
        width: usize,
        height: usize,
        // First dimension is X (Column), second is Y (Row)
        map: Vec<Vec<Square>>,

        // Tracked in insert_sheet
        max_color: usize,
    }

    //
    pub struct Point {
        x: usize,
        y: usize,
    }

    // A single unit in the canvas
    struct Square {
        color: usize, // No negative colors
        position: Point,
    }

    impl Canvas {
        pub fn new(w: usize, h: usize) -> Canvas {
            let mut m = Vec::<Vec<Square>>::with_capacity(w);
            for x in 0..w {
                m.push(Vec::<Square>::with_capacity(h));
                for y in 0..h {
                    m[x].push(Square {color: 0, position: Point { x: x, y: y }});
                }
            }

            let canvas = Canvas { width: w, height: h, map: m, max_color: 0 };

            assert_eq!(canvas.width, canvas.map.len());
            assert_eq!(canvas.height, canvas.map[0].len());

            canvas
        }

        pub fn new_from_string(s: String) -> Canvas {
            // Separate input by \n
            let input: Vec<&str> = s.split('\n').collect();

            // First line is dimensions
            let dimensions: Vec<&str> = input[0].split(' ').collect();
            let width = match usize::from_str(dimensions[0]) {
                Ok(x) => x,
                Err(e) => {
                    println!("error parsing width: {}", e);
                    0
                }
            };
            let height = match usize::from_str(dimensions[1]) {
                Ok(x) => x,
                Err(e) => {
                    println!("error parsing height: {}", e);
                    0
                }
            };

            println!("Dimensions: {}x{}", width, height);

            let mut c = Canvas::new(width, height);

            // Following lines are sheets
            for index in 1..input.len()-1 {
                c.insert_sheet_from_str(input[index]);
            }

            c
        }

        pub fn insert_sheet(&mut self, position: Point, width: usize, height: usize, c: usize) {
            if c > self.max_color {
                self.max_color = c;
            }
            for x in (position.x)..(position.x + width) {
                for y in (position.y)..(position.y + height) {
                    let s: Square = Square {color: c, position: Point {x: x, y: y}};
                    self.map[x][y] = s;
                }
            }
        }

        pub fn color_at(&self, position: Point) -> usize {
            self.map[position.x][position.y].color
        }

        pub fn area_of_colors(&self) -> String {
            // Count of color frequency where the index is the color
            let mut colors: Vec<usize> = Vec::<usize>::with_capacity(self.max_color);
            for i in 0..self.max_color+1 {
                colors.push(0);
            }

            let mut area_string: String = String::new();

            for x in 0..self.width {
                for y in 0..self.height {
                    let color = self.map[x][y].color;
                    colors[color] += 1;
                }
            }

            for color in 0..colors.len() {
                let count = colors[color];
                if count != 0 {
                    area_string.push_str(&format!("{} {}\n", color, count));
                }
            }

            area_string
        }

        fn insert_sheet_from_str(&mut self, s: &str) {
            let line: Vec<&str> = s.split(' ').collect();
            if line.len() != 5 {
                panic!("Expected 5 inputs, received {}.", line.len());
            } else {
                let color = match usize::from_str(line[0]) {
                    Ok(x) => x,
                    Err(e) => {
                        println!("error parsing color: {}.", e);
                        0
                    }
                };
                let x = match usize::from_str(line[1]) {
                    Ok(x) => x,
                    Err(e) => {
                        println!("error parsing x: {}.", e);
                        0
                    }
                };
                let y = match usize::from_str(line[2]) {
                    Ok(x) => x,
                    Err(e) => {
                        println!("error parsing y: {}.", e);
                        0
                    }
                };
                let w = match usize::from_str(line[3]) {
                    Ok(x) => x,
                    Err(e) => {
                        println!("error parsing width: {}.", e);
                        0
                    }
                };
                let h = match usize::from_str(line[4]) {
                    Ok(x) => x,
                    Err(e) => {
                        println!("error parsing height: {}.", e);
                        0
                    }
                };
                self.insert_sheet(Point {x: x, y: y}, w, h, color);
            }
        }

        /*
        fn description(&self) -> String {
            "Broken"
        }

        fn print(&self) {

        }
        */
    }
}
