// Thomas Ring
// June 22, 2015
// A simple add, remove, print todo list
// http://www.reddit.com/r/dailyprogrammer/comments/39ws1x/20150615_challenge_218_easy_todo_list_part_1/
// main.rs

extern crate todo_part1;
use todo_part1::todo::{ToDoList};

fn main() {
    test_list();
}

pub fn test_list() {
    let mut new_list = ToDoList::new();
    new_list.addItem("Take a shower".to_string());
    new_list.addItem("Go to work".to_string());
    new_list.viewList();

    new_list.addItem("Buy a new phone".to_string());
    new_list.deleteItem("Go to work".to_string());
    new_list.viewList();
}
