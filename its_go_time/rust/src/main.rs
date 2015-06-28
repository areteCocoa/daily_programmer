// Thomas Ring
// June 24, 2015
// Simulates a Go! board and determines the best move to remove the most opponent tiles
// http://www.reddit.com/r/dailyprogrammer/comments/3axjac/20150624_challenge_220_intermediate_its_go_time/
// Submission link
// lib.rs

extern crate its_go_time;
use its_go_time::go::{GameBoard, Point};

fn main() {
    let board = GameBoard::new_with_input(test1());
    board.print();
    board.blank_tiles_around_tile_group(&Point::new(3, 1));
}

fn test1() -> &'static str {
    "7 5
b
 bbbbb
bbwwwwb
bww wb
 bwwwwb
  bbbbb"
}

/*
fn test2() {

}
*/
