// Thomas Ring
// June 22, 2015
// Inputs a sentence and returns each word alphabetized.
// http://www.reddit.com/r/dailyprogrammer/comments/3aqvjn/20150622_challenge_220_easy_mangling_sentences/
// main.rs

fn main() {
    let hello = "hello world";
    let mangled_hello = mangle(hello);
    println!("{}", mangled_hello);
}

fn mangle(s: &str) -> &str {
    let result = String::new();
    for word in s.to_string().split(' ') {

    }
}

fn alphabetize(s: &str) -> &str {
    let mut alphabetized = String::with_capacity(s.len());
    let special_chars = vec!['-', ',', '\''];

    for special in special_chars {
        for character in s.chars() {
            match word.find(spec) {
                Some(x) => {
                    alphabetized.insert(x, spec);
                },
                None =>{
                    // Do nothing!
                }
            }
        }
        s.replace(special, ' ');
    }

    let words = s.split(' ')


    let mut words = s.split(' ');
    for word in words.iter() {
        for spec in special_chars {
            match word.find(spec) {
                Some(x) => {
                    alphabetized()
                },
                None => {
                    // Do nothing!
                }
            }
        }
    }

    for c in s.chars() {
        for special in special_chars {
            match c
        }
    }


}
