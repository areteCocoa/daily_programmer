// Thomas Ring
// June 29, 2015
// The Word Snake problem.
// main.rs

extern crate wordsnake;
use wordsnake::{CharField};

fn main() {
    let field = CharField::new_with_sentence(input_one());
    field.print();
}

fn input_one() -> &'static str {
    "SHENANIGANS SALTY YOUNGSTER ROUND DOUBLET TERABYTE ESSENCE"
}
