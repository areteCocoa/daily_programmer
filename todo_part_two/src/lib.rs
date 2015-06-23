// Thomas Ring
// June 22, 2015
// A slightly more complicated todo list with add, remove, modify, categories and save/load
// http://www.reddit.com/r/dailyprogrammer/comments/3a64hq/20150617_challenge_219_intermediate_todo_list/
// lib.rs

// "(don't take this too literally, do it how you would like to do it)"
// thank god
// Challenge requests camel case, not snake case >_>
//#![allow(non_snake_case)]

pub mod todo {
    use std::fs::File;
    use std::io::Read;
    use std::io::Write;
    use std::path::Path;
    use std::ascii::AsciiExt;

    struct ToDoItem {
        data: String,
        categories: Vec<String>
    }

    impl ToDoItem {
        pub fn new(name: String) -> ToDoItem {
            let item = ToDoItem {data: name, categories: Vec::<String>::new()};

            item
        }

        pub fn change_name(&mut self, new_name: String) {
            self.data = new_name;
        }

        pub fn add_category(&mut self, category: &str) {
            self.categories.push(category.to_string());
        }

        pub fn name(&self) -> &str {
            &self.data
        }

        pub fn is_item(&self, name: &String) -> bool {
            (self.data == *name)
        }

        pub fn is_category(&self, category: &str) -> bool {
            let mut iter = self.categories.iter();
            loop {
                match iter.next() {
                    Some(x) => {
                        if *x == category {
                            return true;
                        }
                    }
                    None => { break }
                }
            }
            false
        }

        pub fn print(&self) {
            print!("{}", self.data);
        }
    }

    pub struct ToDoList {
        items: Vec<ToDoItem>,
        categories: Vec<String>,
        filename: String
    }

    impl ToDoList {
        pub fn new() -> ToDoList {
            let list = ToDoList::new_from_file("todo.txt".to_string());

            list
        }

        // Default constructor
        pub fn new_from_file(filename: String) -> ToDoList {
            let mut list = ToDoList {items: Vec::<ToDoItem>::new(), categories: Vec::<String>::new(), filename: filename};

            list.load_from_file();

            list
        }

        fn load_from_file(&mut self) {
            // Not the ideal way to deal with file input, oh well
            let mut file: File = match File::open(Path::new(&self.filename)) {
                Ok(file) => {
                    file
                }
                Err(..) => {
                    panic!("Error reading file {}!", self.filename);
                }
            };

            let mut input_string = String::new();
            let buffer_length = file.read_to_string(&mut input_string);

            // Format for items is:
            // ITEM NAME-_-CATEGORY 1-_-CATEGORY 2
            for item_string in input_string.split('\n') {
                let mut item_vec: Vec<&str> = item_string.split("-_-").collect();
                let item_name = item_vec.remove(0);
                self.add_item(item_name.to_string(), item_vec);
            }
        }

        pub fn add_item(&mut self, item: String, categories: Vec<&str>) {
            let mut td_item = ToDoItem::new(item);
            for category in categories {
                let lowercase_cat = category.to_ascii_lowercase();
                td_item.add_category(&lowercase_cat);
                self.add_category(&lowercase_cat);
            }
            self.items.push(td_item);

            self.save();
        }

        pub fn delete_item(&mut self, item: String) {
            for i in 0..self.items.len()-1 {
                if self.items[i].is_item(&item) {
                    self.items.remove(i);
                }
            }
        }

        pub fn change_item_name(&mut self, item: String, new_name: String) {
            match self.item_for_string(item) {
                Some(x) => {
                    x.change_name(new_name);
                }
                None => {
                    // Item does not exist
                }
            }

            self.save();
        }

        pub fn save(&self) {
            let mut output_string = String::new();
            for item in self.items.iter() {

                output_string.push_str(item.name());
                for category in self.categories.iter() {
                    if item.is_category(category) {
                        output_string.push_str("-_-");
                        output_string.push_str(category);
                    }
                }
                output_string.push('\n');
            }
            // println!("Saving string: {}", output_string);
            let output = output_string.as_bytes();

            let mut file = match File::create(Path::new(&self.filename)) {
                Ok(x) => {
                    x
                },
                Err(..) => {
                    panic!("Error creating file to save list.");
                }
            };
            match file.write(output) {
                Ok(..) => {
                    println!("Successfully saved!");
                }
                Err(..) => {
                    println!("Error saving list.");
                }
            }
        }

        fn item_for_string(&mut self, name: String) -> Option<&mut ToDoItem> {
            for item in self.items.iter_mut() {
                if item.is_item(&name) {
                    return Some(item);
                }
            }
            return None;
        }

        fn items_for_category(&self, category: &str) -> Vec<&ToDoItem> {
            let mut iter = self.items.iter();
            let mut items = Vec::<&ToDoItem>::new();
            loop {
                match iter.next() {
                    Some(x) => {
                        if (*x).is_category(category) {
                            items.push(x);
                        }
                    }
                    None => {break}
                }
            }
            items
        }

        fn add_category(&mut self, category: &str) {
            for cat in self.categories.iter() {
                if cat == category { // It already exists in the thing
                    return;
                }
            }
            // println!("Adding category: {}", category);
            self.categories.push(category.to_string());
        }

        pub fn print(&self) {
            for category in self.categories.iter() {
                println!("===== {} =====", category.to_ascii_uppercase());
                for item in self.items_for_category(category).iter() {
                    item.print();
                    println!("");
                }
                println!("");
            }
        }
    }
}
