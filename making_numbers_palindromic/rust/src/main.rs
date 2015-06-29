// Thomas Ring
// June 08, 2015
// Finds number palindromes.
// https://www.reddit.com/r/dailyprogrammer/comments/38yy9s/20150608_challenge_218_easy_making_numbers/
// https://www.reddit.com/r/dailyprogrammer/comments/38yy9s/20150608_challenge_218_easy_making_numbers/cslvjv4
// main.rs

fn main() {
    palindromic(11);
    palindromic(68);
    palindromic(123);
    palindromic(286);
    palindromic(196196871);
}

fn palindromic(n: usize) {
    let mut all_numbers = Vec::<usize>::new();
    all_numbers.push(n);
    loop {
        let current = *all_numbers.last().unwrap();
        let palindrome = reverse_number(current);
        let result = current + palindrome;
        if all_numbers.contains(&palindrome) {
            break;
        }
        all_numbers.push(result);
    }
    let steps = all_numbers.len()-1;
    println!("{} gets palindromic after {} steps: {}", n, steps, all_numbers.last().unwrap());
}

fn reverse_number(n: usize) -> usize {
    let s = format!("{}", n);
    let r = reverse_string(s);
    let number = match r.parse::<usize>() {
        Ok(x) => {
            x
        },
        Err(e) => {
            println!("Error parsing number: {}", e);
            0
        },
    };
    number
}

fn reverse_string(s: String) -> String {
    let mut bytes = s.to_string().into_bytes();
    bytes.reverse();
    let reverse = match String::from_utf8(bytes.to_vec()) {
            Ok(x) => x,
            Err(e) => {
                println!("Error reversing string, {}", e);
                "".to_string()
            }
    };
    reverse
}
