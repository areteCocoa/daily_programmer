// Thomas Ring
// June 24, 2015
// Simulates a Go! board and determines the best move to remove the most opponent tiles
// http://www.reddit.com/r/dailyprogrammer/comments/3axjac/20150624_challenge_220_intermediate_its_go_time/
// Submission link
// lib.rs

/*
    Player One - White - 1
    Player Two - Black - 2
*/

pub mod go {
    /*
        a vector of the pieces. 0 is no player, 1 is p1, 2 is p2
    */
    pub struct GameBoard {
        board: Vec<Vec<u8>>,
        size: Size,
        current_player: u8,
    }

    pub struct Size {
        width: usize,
        height: usize,
    }

    pub struct Point {
        x: usize,
        y: usize,
    }

    enum Direction {
        Left,
        Up,
        Right,
        Down,
    }

    /// Returns a vector of all directions
    fn all_directions() -> Vec<Direction> {
        vec![Direction::Left, Direction::Up, Direction::Right, Direction::Down]
    }

    impl GameBoard {
        /*
            A constuctor for 5x5 size
        */
        pub fn new() -> GameBoard {
            let game = GameBoard::new_with_size(5, 5);

            game
        }

        pub fn new_with_size(width: usize, height: usize) -> GameBoard {
            let mut game = GameBoard {board: Vec::<Vec<u8>>::with_capacity(width),
                size: Size {width: width, height: height}, current_player: 0};

            for column in 0..width {
                game.board.push(Vec::<u8>::with_capacity(height));
                for row in 0..height {
                    game.board[column].push(0);
                }
            }

            game
        }

        /*
            The best constructor for the reddit problem. Parses input.
        */
        pub fn new_with_input(input: &str) -> GameBoard {
            let mut input_lines: Vec<&str> = input.split('\n').collect();
            let size_input: &str = input_lines.remove(0);
            let size_strings: Vec<&str> = size_input.split(' ').collect();
            let width = match size_strings[0].parse::<u8>() {
                Ok(x) => x,
                Err(e) => {
                    println!("Error parsing width: {}.", e);
                    0
                }
            };
            let height = match size_strings[1].parse::<u8>() {
                Ok(x) => x,
                Err(e) => {
                    println!("Error parsing height: {}.", e);
                    0
                }
            };

            let mut game_board = GameBoard::new_with_size(width as usize, height as usize);

            let player_line: &str = input_lines.remove(0);
            game_board.current_player = match player_line {
                "w" | "W" => 1,
                "b" | "B" => 2,
                _ => 0,
            };

            for y in 0..game_board.size.height {
                let mut input_chars: Vec<char> = input_lines[y].chars().collect(); // get the input for line y
                while input_chars.len() < game_board.size.width { // ensure it's the correct length
                    input_chars.push('a'); // blank character, gets converted to 0
                }
                for x in 0..game_board.size.width {
                    game_board.board[x][y] = GameBoard::number_for_char(input_chars[x]);
                }
            }

            game_board
        }

        pub fn best_play(&self) -> Point {
            let mut best_point = Point {x: 0, y: 0};
            let mut best: u8 = 0;

            for x in 0..self.size.width-1 {
                for y in 0..self.size.height-1 {
                    let p = Point {x: x, y: y};
                    let r = self.tiles_removed_with_play(&p);
                    if r > best {
                        best_point = p;
                        best = r;
                    }
                }
            }

            best_point
        }

        /*
            Returns the number of tiles removed with tile placement
            at Point point for current_player.
        */
        fn tiles_removed_with_play(&self, point: &Point) -> u8 {
            // If is already taken, won't remove any tiles (invalid play)
            if self.board[point.x][point.y] != 0 {
                return 0;
            }


            // See if tiles inside can be removed
            // Return number of tiles that can be removed

            0
        }

        /// Returns a vector of all the tiles in the group
        fn group_for_tile(&self, point: &Point) -> Vec<Point> {
            let mut tiles = Vec::<Point>::new();

            tiles
        }

        /// Returns a vector of tiles that validate a group.
        pub fn blank_tiles_around_tile_group(&self, point: &Point) -> Vec<Point> {
            let mut blanks = Vec::<Point>::new();

            let blank_directions = self.neighbors(0, point, None);
            let blank_points = self.points_from_directions(point, blank_directions);

            blanks
        }

        /// Return a vector of points from tiles in directions from point.
        fn points_from_directions(&self, point: &Point, directions: Vec<Direction>) -> Vec<Point> {
            let mut points = Vec::<Point>::new();

            for d in directions {
                points.push(self.point_from_direction(point, d));
            }

            points
        }

        /// Returns a point for a direction from another point
        fn point_from_direction(&self, point: &Point, direction: Direction) -> Point {
            match direction {
                Direction::Left => Point::new(point.x-1, point.y),
                Direction::Up => Point::new(point.x, point.y-1),
                Direction::Right => Point::new(point.x+1, point.y),
                Direction::Down => Point::new(point.x, point.y+1),
            }
        }

        /// Finds a tile's neighbors
        fn neighbors(&self, color: u8, point: &Point, exclude: Option<Direction>) -> Vec<Direction> {
            let mut neighbors = Vec::<Direction>::new();
            let directions = all_directions();
            for d in directions {
                match self.get_tile_at_direction(point, &d) {
                    Some(x) => {
                        if x == color {
                            neighbors.push(d);
                        }
                    },
                    None => {},
                };
            }

            neighbors
        }

        fn get_tile_at_direction(&self, point: &Point, direction: &Direction) -> Option<u8> {
            let tile = match *direction {
                Direction::Left => {
                    if point.x > 0 {
                        return Some(self.board[point.x-1][point.y]);
                    }
                    None
                },
                Direction::Up => {
                    if point.y > 0 {
                        return Some(self.board[point.x][point.y-1]);
                    }
                    None
                },
                Direction::Right => {
                    if point.x < self.size.width {
                        return Some(self.board[point.x+1][point.y]);
                    }
                    None
                },
                Direction::Down => {
                    if point.y < self.size.height {
                        return Some(self.board[point.x][point.y+1]);
                    }
                    None
                },
            };

            tile
        }

        /// Prints board vector as u8s
        pub fn print_raw(&self) {
            println!("=== BOARD ===");
            for row in 0..self.size.height {
                for column in self.board.iter() {
                    print!("{} ", column[row]);
                }
                println!("");
            }
            println!("");
        }

        /// Prints board vector as readable b's, w's and *'s.
        pub fn print(&self) {
            println!("=== BOARD ===");
            for row in 0..self.size.height {
                for column in self.board.iter() {
                    print!("{} ", GameBoard::char_for_number(column[row]));
                }
                println!("");
            }
            println!("");
        }

        fn char_for_number(number: u8) -> char {
            match number {
                0 => '*',
                1 => 'w',
                2 => 'b',
                _ => 'e',
            }
        }

        fn number_for_char(c: char) -> u8 {
            match c {
                ' ' => 0,
                'w' | 'W' => 1,
                'b' | 'B' => 2,
                _ => 0,
            }
        }
    }

    impl Point {
        pub fn new(x: usize, y: usize) -> Point {
            Point {x: x, y: y}
        }

        pub fn print(&self) {
            print!("({}, {})", self.x, self.y);
        }
    }
}
