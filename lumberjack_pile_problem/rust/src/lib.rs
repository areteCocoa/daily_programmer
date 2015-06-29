// Thomas Ring
// June 01, 2015
// A program to simulate a lumberjack stacking wood efficiently.
// https://www.reddit.com/r/dailyprogrammer/comments/3840rp/20150601_challenge_217_easy_lumberjack_pile/
// (submission link)
// lib.rs

pub struct LumberArea {
    lumber: Vec<u64>, // u64 in case there's ever a MASSIVE lumber storage...
    width: usize, // For display purposes
    lowest_value: u64, // The lowest value in the lumber vector
}

impl LumberArea {
    pub fn new() -> LumberArea {
        let l = LumberArea { lumber: Vec::<u64>::new(), width: 0, lowest_value: 0 };

        l
    }

    pub fn new_from_input(input: &str) -> LumberArea {
        let mut l = LumberArea::new();
        l.lowest_value = 10000; // So it can be set lower later

        // Break input by lines
        let mut input_lines: Vec<&str> = input.split('\n').collect();
        // First line is width and height (it's always a square). Parse to width field.
        let input_size: &str = input_lines.remove(0);
        l.width = match input_size.parse::<usize>() {
            Ok(x) => {
                x
            },
            Err(e) => {
                println!("Error parsing width to usize: {}", e);
                0
            }
        };

        // Next line is number of logs to be added. Don't parse yet,
        // we need all the other information first.
        let input_logs = input_lines.remove(0);
        let logs = match input_logs.parse::<u64>() {
            Ok(x) => {
                x
            },
            Err(e) => {
                println!("Error parsing log count to u64: {}", e);
                0
            }
        };

        // Iterate through the rest of the input for the vector values
        for i in input_lines { // in each of the remaining lines
            for input_value in i.split(' ') { // iterate through each number
                let option = input_value.parse::<u64>();
                if let Ok(value) = option {
                    if value < l.lowest_value {
                        l.lowest_value = value;
                    }
                    l.lumber.push(value);
                }
            }
        }

        // Finally add logs
        l.add_logs(logs);
        l
    }

    /// Adds logs number of logs to the lumber pile.
    pub fn add_logs(&mut self, logs: u64) {
        let mut remaining = logs;
        while remaining > 0 {
            for l in self.lumber.iter_mut() {
                if *l <= self.lowest_value && remaining > 0 {
                    *l += (1 as u64);
                    remaining -= 1;
                } else if remaining <= 0 {
                    return; // exit so we don't change lowest_value
                }
            }
            self.lowest_value += 1; // The for loop changes all spots where they are lowest value and
            // the return in the else if stops it from reaching this point if it's not true.
        }
    }

    pub fn print(&self) {
        println!("=== LUMBER AREA ===");
        for index in 0..self.lumber.len() {
            print!("{} ", self.lumber[index]);
            if (index + 1) % self.width == 0 {
                println!("");
            }
        }
    }
}
