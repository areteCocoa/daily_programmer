// Thomas Ring
// June 22, 2015
// A simple add, remove, print todo list
// http://www.reddit.com/r/dailyprogrammer/comments/39ws1x/20150615_challenge_218_easy_todo_list_part_1/
// lib.rs

#![allow(non_snake_case)]

pub mod todo {
    pub struct ToDoList {
        items: Vec<String>,
    }

    impl ToDoList {
        pub fn new() -> ToDoList {
            let list = ToDoList {items: Vec::<String>::new()};

            list
        }

        pub fn addItem(&mut self, item: String) {
            println!("Adding item {}", item);
            self.items.push(item);
        }

        pub fn deleteItem(&mut self, item: String) {
            for i in 0..self.items.len()-1 {
                if item == self.items[i] {
                    println!("Removing item!");
                    self.items.remove(i);
                }
            }
        }

        pub fn viewList(&self) {
            for item in 0..self.items.len() {
                println!("{}. {}", item+1, self.items[item]);
            }
        }
    }
}
