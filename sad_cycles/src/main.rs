// Thomas Ring
// May 20, 2015
// main.rs
// Finds the sad cycle for a given base and starting point and returns it as a Vec<i32>

#![feature(collections)]

fn main() {
    let base = 2;
    let start = 12;
    let example_cycle = sad_cycle(base, start);
    print_cycle(example_cycle);

    let base1 = 6;
    let start1 = 2;
    let cycle1 = sad_cycle(base1, start1);
    print_cycle(cycle1);

    let base2 = 7;
    let start2 = 7;
    let cycle2 = sad_cycle(base2, start2);
    print_cycle(cycle2);

    let base3 = 3;
    let start3 = 14;
    let cycle3 = sad_cycle(base3, start3);
    print_cycle(cycle3);

    let base4 = 11;
    let start4 = 2;
    let cycle4 = sad_cycle(base4, start4);
    print_cycle(cycle4);
}

fn sad_cycle(base: u32, starting_number: i64) -> Vec<i64> {
    let mut number = starting_number;
    let mut numbers = Vec::<i64>::new();

    while !numbers.contains(&number) {
        numbers.push(number);
        number = sum_digits(number, base);
    }

    let start_index = numbers.position_elem(&number).unwrap_or(0);

    let slice = numbers.split_at(start_index).1;
    let mut cycle = Vec::<i64>::new();
    cycle.push_all(slice);

    cycle
}

fn sum_digits(number: i64, base: u32) -> i64 {
    let mut n = number;
    let mut sum = 0;
    while n > 0 {
        sum += (n % 10).pow(base);
        n = n / 10;
    }
    sum
}

fn print_cycle(cycle: Vec<i64>) {
    for number in cycle {
        print!(" {}", number);
    }
    println!(".");
}
