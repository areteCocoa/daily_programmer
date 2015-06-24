use std::ascii::AsciiExt;

fn main() {
    let example_name_game_string = name_game("Lincoln".to_string());

    println!("{}", example_name_game_string);
}

fn name_game(name: String) -> String {
    let mut game = String::new();

    let vowels = vec!['a', 'e', 'i', 'o', 'u'];
    let first_char = name.chars().next().unwrap_or('a').to_ascii_lowercase();
    let mut is_vowel = false;

    for index in 0..vowels.len() {
        let v = vowels[index];
        if first_char == v {
            is_vowel = true;
            break;
        }
    }

    let mut first_line_edit = (&name).to_string();
    let mut second_line_edit = (&name).to_string();
    let mut third_line_edit = (&name).to_string();

    if !is_vowel {
        first_line_edit.remove(0);
        second_line_edit.remove(0);
        third_line_edit.remove(0);

        first_line_edit.insert(0, 'B');
        second_line_edit.insert(0, 'F');
        third_line_edit.insert(0, 'M');
    }

    game.push_str(&format!("{}, {} bo {},\n", name, name, first_line_edit));
    game.push_str(&format!("Bonana fanna fo {},\n", second_line_edit));
    game.push_str(&format!("Fee fy mo {},\n", third_line_edit));
    game.push_str(&format!("{}!", name));

    game
}
