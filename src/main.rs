mod logic;
mod parser;
mod common_functions;
mod matrix;
mod interpreter;

use crate::logic::*;
use crate::parser::*;
use crate::matrix::*;

fn main() {
    matrix!(cool, "a b c|a c b");
    dbg!(cool);
}
