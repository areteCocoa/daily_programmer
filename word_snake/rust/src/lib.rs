// Thomas Ring
// June 29, 2015
// The Word Snake problem.
// lib.rs

/// CharField represents a 2D field of characters.
pub struct CharField<'a> {
    field: Vec<Vec<char>>,
    words: Vec<&'a str>,
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

impl<'a> CharField<'a> {
    /// Returns a CharField with no words in it and a 10x10 field
    pub fn new() -> CharField<'a> {
        let wf = CharField { field: vec![vec![' '; 10]; 10], words: Vec::<&str>::new() };

        wf
    }

    /// Returns a CharField given the sentence &str
    pub fn new_with_sentence(sentence: &str) -> CharField<'a> {
        let mut word_field = CharField::new();

        let mut position = Position {x: 0, y: 0};
        let mut direction = Direction::Up;
        for word in sentence.split_whitespace() {
            direction = match direction {
                Direction::Left | Direction::Right => {
                    Direction::Down
                },
                Direction::Up | Direction::Down => {
                    Direction::Right
                }
            };
            position = word_field.insert_string(position, &direction, word);
        }

        word_field
    }

    /// Prints the CharField to the println stream
    pub fn print(&self) {
        println!("=== CharField ===");
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

    // Inserts char c at Position p.
    fn insert_char(&mut self, p: &Position, c: char) {
        if self.field[p.y][p.x] == ' ' {
            self.field[p.y][p.x] = c;
        }
    }

    fn add_column(&mut self) {
        for row in self.field.iter_mut() {
            row.push(' ');
        }
    }

    fn add_row(&mut self) {
        let width = self.field[0].len();
        self.field.push(vec![' '; width]);
    }

    fn width(&self) -> usize {
        self.field[0].len()
    }

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
