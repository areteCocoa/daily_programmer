// Thomas Ring
// June 22, 2015
// Inputs a sentence and returns each word alphabetized.
// http://www.reddit.com/r/dailyprogrammer/comments/3aqvjn/20150622_challenge_220_easy_mangling_sentences/
// main.rs

use std::ascii::AsciiExt;

fn main() {
    let input1 = "This challenge doesn't seem so hard.";
    let input2 = "There are more things between heaven and earth, Horatio, than are dreamt of in your philosophy.";
    let inputs = vec![input1, input2];

    let cinput1 = "Eye of Newt, and Toe of Frog, Wool of Bat, and Tongue of Dog.";
    let cinput2 = "Adder's fork, and Blind-worm's sting, Lizard's leg, and Howlet's wing.";
    let cinput3 = "For a charm of powerful trouble, like a hell-broth boil and bubble.";
    let cinputs = vec![cinput1, cinput2, cinput3];

    for i in inputs {
        let m = mangle(i);
        println!("{}", m);
    }

    for i in cinputs {
        let m = mangle(i);
        println!("{}", m);
    }
}

fn mangle(s: &str) -> String {
    let mut result = String::new();

    for word in s.split(" ") {
        let alpha = alphabetize(word);
        result.push_str(&alphabetize(word));
        result.push(' ');
    }

    result
}

// Not "true" alphabetize, capitalizes first letter if it was before
fn alphabetize(s: &str) -> String {
    // Check if first character is uppercase
    let first_upper = s.chars().next().unwrap_or(' ').is_uppercase();
    // Prepare final string
    let mut alphabetized = String::with_capacity(s.len());

    // create vector of u8 with lowercased string, sort
    let mut bytes = s.to_ascii_lowercase().as_bytes().to_vec();
    bytes.sort_by(|a, b| a.cmp(b));

    // Convert and match to String
    let mut result = match String::from_utf8(bytes.to_vec()) {
        Ok(r) => {
            r
        }
        Err(e) => panic!("Error!"),
    };

    // Set the upper
    if first_upper {
        let first_char = result.chars().next().unwrap_or(' ').to_ascii_uppercase();
        result.insert(0, first_char);
    }

    result
}
