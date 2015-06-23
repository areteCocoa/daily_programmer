// Thomas Ring
// June 22, 2015
// A slightly more complicated todo list with add, remove, modify, categories and save/load
// http://www.reddit.com/r/dailyprogrammer/comments/3a64hq/20150617_challenge_219_intermediate_todo_list/
// main.rs

extern crate todo_part_two;
use todo_part_two::todo::{ToDoList};

fn main() {
    //test_list();
    let list = ToDoList::new();
    list.print();
}

pub fn test_list() {
    let mut new_list = ToDoList::new();
    new_list.add_item("Take a shower".to_string(), vec!["Personal"]);
    new_list.add_item("Go to work".to_string(), vec![]);
    new_list.print();

    new_list.add_item("Buy a new phone".to_string(), vec!["Recreational", "Personal"]);
    new_list.delete_item("Go to work".to_string());
    new_list.print();

    new_list.save();
}
