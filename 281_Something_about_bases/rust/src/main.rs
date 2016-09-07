/*
 *
 * Thomas Ring
 * Something about bases
 * September 4, 2016
 * main.rs
 *
 */

fn main() {
    base("1");
    base("21");
    base("ab3");
    base("ff");
}

// Prints the largest base and the base 10 value of the number input
fn base(number: &'static str) {
    let b = largest_base_from_string(number.to_string());
    let value = value_in_base(number.to_string(), b);
    println!("base {} => {}", b, value);
}

// Returns the value for the number in the given base
fn value_in_base(number: String, base: u32) -> u32 {
    let mut total = 0;

    let length = number.len() as u32;
    
    for (index, c) in number.chars().enumerate() {
        match c.to_digit(base) {
            Some(d) => {
                let position: u32 = index as u32;
                let position_value = base.pow(length - position - 1);
                let digit_value = position_value * d;
//                println!("{}", digit_value);
                total = total + digit_value;
            },
            _ => {println!("ERROR");}
        }
    }

    return total;
} 

// Takes a string and returns the largest base that string could belong to
fn largest_base_from_string(number_string: String) -> u32 {
    return largest_digit_from_string(number_string) + 1;
}

// Takes a string and returns the largest digit in the string
fn largest_digit_from_string(number_string: String) -> u32 {
    let mut largest = 0;
    for c in number_string.chars() {
        match c.to_digit(16) {
            Some(digit) => {
                if digit > largest {
                    largest = digit;
                }
            },
            _ => {},
        };
    }
    
    return largest;
}
