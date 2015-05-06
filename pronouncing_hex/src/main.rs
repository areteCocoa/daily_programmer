use std::collections::HashMap;

// Because of some error with types
static SPACE: &'static &'static str = &" ";

fn main() {
    let input = vec![0xF5, 0xB3, 0xE4, 0xBBBB, 0xA0C9]; // Example input

    for n in input {
        let s = format!("{0:x}", n);
        println!("{}", pronounce_hex(s));
    }
}

fn pronounce_hex(hex: String) -> String {
    let mut hex_conversion = String::new();
    let map = get_hash_map();
    for(i, c) in hex.chars().enumerate() {
        let s = match map.get(&c.to_string()) {
            Some(x) => x,
            None => SPACE
        };

        hex_conversion.push_str(*s);
        if i % 2 == 0 {
            hex_conversion.push_str("-");
        } else if c != '0' {
            hex_conversion.push_str(" ");
        }

        if i % 2 == 1 && hex.len() - 1 > i {
            hex_conversion.push_str("bitey ");
        }
    }
    hex_conversion
}

fn get_hash_map() -> HashMap<String, &'static str> {
    let mut hex_map = HashMap::new();
    hex_map.insert("0".to_string(), "");
    hex_map.insert("1".to_string(), "one");
    hex_map.insert("2".to_string(), "two");
    hex_map.insert("3".to_string(), "three");
    hex_map.insert("4".to_string(), "four");
    hex_map.insert("5".to_string(), "five");
    hex_map.insert("6".to_string(), "six");
    hex_map.insert("7".to_string(), "seven");
    hex_map.insert("8".to_string(), "eight");
    hex_map.insert("9".to_string(), "nine");
    hex_map.insert("a".to_string(), "atta");
    hex_map.insert("b".to_string(), "bibbity");
    hex_map.insert("c".to_string(), "city");
    hex_map.insert("d".to_string(), "dickety");
    hex_map.insert("e".to_string(), "ebbity");
    hex_map.insert("f".to_string(), "fleventy");
    hex_map
}
