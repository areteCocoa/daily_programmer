// Thomas Ring
// June 29, 2015
// The Word Snake problem.
// lib.rs

/// CharField represents a 2D field of characters.
pub struct CharField<> {
    field: Vec<Vec<char>>,
}

struct Position {
    x: usize,
    y: usize,
}

enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl CharField {
    /// Returns a CharField with no words in it and a 10x10 field
    pub fn new() -> CharField {
        let wf = CharField { field: vec![vec![' '; 10]; 10] };

        wf
    }

    /// Returns a CharField given the sentence &str
    pub fn new_with_sentence(sentence: &str) -> CharField {
        let mut char_field = CharField::new();

        let mut position = Position {x: 0, y: 0};
        let mut direction = Direction::Up;
        for word in sentence.split_whitespace() {
            direction = match direction {
                Direction::Left | Direction::Right => {
                    match char_field.insert_is_valid(&position, &Direction::Up, word) {
                        true => Direction::Up,
                        false => Direction::Down,
                    }
                },
                Direction::Up | Direction::Down => {
                    match char_field.insert_is_valid(&position, &Direction::Left, word) {
                        true => Direction::Left,
                        false => Direction::Right,
                    }
                }
            };
            position = char_field.insert_string(position, &direction, word);
        }

        char_field
    }

    /// Prints the CharField to the println stream
    pub fn print(&self) {
        for row in self.field.iter() {
            // For each character in the row
            for c in row.iter() {
                print!("{}", c);
            }
            println!("");
        }
    }

    /// Inserts the string w into the CharField starting at position p in Direction d.
    fn insert_string(&mut self, p: Position, d: &Direction, w: &str) -> Position {
        let mut pos = Position {x: p.x, y: p.y};
        for c in w.chars() {
            if pos.x >= self.width() {
                self.add_column();
            }
            if pos.y >= self.height() {
                self.add_row();
            }

            self.insert_char(&pos, c);

            pos.move_direction(d);
        }
        pos.move_opposite_direction(d); // For initial value
        pos
    }

    // Inserts char c at Position p
    fn insert_char(&mut self, p: &Position, c: char) {
        if self.field[p.y][p.x] == ' ' {
            self.field[p.y][p.x] = c;
        }
    }

    fn insert_is_valid(&self, p: &Position, d: &Direction, w: &str) -> bool {
        let x = p.x as i8; // Use i8 here and not Position/usize because it is possible
        let y = p.y as i8; // to have a negative value for the end value causing arithmitic OF
        let mut position = Position { x: p.x, y: p.y };
        let length = w.len();
        let (x_iter, y_iter): (i8, i8) = match *d {
            Direction::Left => (-1, 0),
            Direction::Up   => (0, -1),
            Direction::Right=> (1, 0),
            Direction::Down => (0, 1),
        };
        let x_len = x_iter * (length as i8); // x distance traversed
        let y_len = y_iter * (length as i8); // y distance traversed
        let end_x = x + (x_len as i8); // X endpoint
        let end_y = y + (y_len as i8); // Y endpoint
        // check to make sure we're still in the top/left boundaries
        if end_x < 0 || end_y < 0 {
            return false;
        }

        for _ in 0..length {
            position.move_direction(d);
            if let Some(row) = self.field.get(position.y) {
                if let Some(c) = row.get(position.x) {
                    if *c != ' ' {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Adds a column to the field.
    fn add_column(&mut self) {
        for row in self.field.iter_mut() {
            row.push(' ');
        }
    }

    /// Adds a row to the field.
    fn add_row(&mut self) {
        let width = self.field[0].len();
        self.field.push(vec![' '; width]);
    }

    /// Returns the width of the field.
    fn width(&self) -> usize {
        self.field[0].len()
    }

    /// Returns the height of the field.
    fn height(&self) -> usize {
        self.field.len()
    }
}

impl Position {
    fn move_direction(&mut self, direction: &Direction) {
        match *direction {
            Direction::Left => self.x -= 1,
            Direction::Up   => self.y -= 1,
            Direction::Right=> self.x += 1,
            Direction::Down => self.y += 1,
        };
    }

    fn move_opposite_direction(&mut self, direction: &Direction) {
        match *direction {
            Direction::Left => self.x += 1,
            Direction::Up   => self.y += 1,
            Direction::Right=> self.x -= 1,
            Direction::Down => self.y -= 1,
        };
    }
}
