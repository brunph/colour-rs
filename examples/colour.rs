extern crate colour_rs;

use colour_rs::lib::{push_colour, pop_colour};
use colour_rs::lib::Colours::{BLUE};

fn main() {
    push_colour(BLUE);
    println!("LIGHTBLUE");
    pop_colour();
}