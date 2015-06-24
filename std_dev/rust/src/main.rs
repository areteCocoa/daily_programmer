//
// Thomas Ring
// May 12, 2015
// Takes a Vec<f64> of inputs and calculates the population standard deviation. Cool.
// main.rs

#![feature(core)]

extern crate core;
use core::str::FromStr;

fn main() {
    let sample_inputs: Vec<f64> = vec![5.0, 6.0, 11.0, 13.0, 19.0, 20.0, 25.0, 26.0, 28.0, 37.0];
    let sample_std_deviation = std_dev(sample_inputs);

    let input_one = "266 344 375 399 409 433 436 440 449 476 502 504 530 584 587".to_string();
    let input_two = "809 816 833 849 851 961 976 1009 1069 1125 1161 1172 1178 1187 1208 1215 1229 1241 1260 1373".to_string();

    let std_dev_one = std_dev(parse_string_to_vec(input_one));
    let std_dev_two = std_dev(parse_string_to_vec(input_two));

    println!("Sample standard deviation: {}", sample_std_deviation);
    println!("Std dev one: {}", std_dev_one);
    println!("Std dev two: {}", std_dev_two);
}

fn parse_string_to_vec(num_string: String) -> Vec<f64> {
    let split_vec: Vec<&str> = num_string.rsplit(' ').collect();
    let mut num_vec: Vec<f64> = Vec::<f64>::new();
    for piece in split_vec {
        match f64::from_str(piece) {
            Ok(x) => {num_vec.push(x);},
            Err(..) => {;},
        };

    }

    num_vec
}

fn mean(numbers: &Vec<f64>) -> f64 {
    let mut sum: f64 = 0.0;

    for num in numbers {
        sum += *num;
    }

    sum / (numbers.len() as f64)
}

fn variance(numbers: &Vec<f64>, mean: &f64) -> f64 {
    let mut diff: f64 = 0.0;

    for num in numbers {
        diff += (*num - mean) * (*num - mean);
    }

    diff / (numbers.len() as f64)
}

fn std_dev(numbers: Vec<f64>) -> f64 {
    let mean = mean(&numbers);
    let variance = variance(&numbers, &mean);

    variance.sqrt()
}
