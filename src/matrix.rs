use itertools::Itertools;
use crate::logic::*;
use std::fmt;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::error::ParseError;
use nom::{Finish, IResult};
use crate::parser::{Code, ws};
use crate::matrix;

#[derive(Debug)]
pub struct Matrix {
    content: Vec<Vec<String>>,
    name: String,
    rows: usize,
    columns: usize,
}

impl Portable for Matrix {
    fn port(&self) -> Code {
        let height = self.content.len();
        let width = self.content[0].len();

        (0..height * width)
            .map(|index| get_position(index, height, width))
            .map(|(row, col)| matrix_entry_to_code(&self, col, row))
            .join("")
            .to_string()
    }
}

#[test]
fn test() {
    dbg!({
        matrix!(cool, "1 2 3");
        cool.port()
    });
}

pub fn matrix_parse(matrix: &str, name: &str) -> Matrix {
    if matrix == "" {panic!("Matrix '{name}' is empty")}

    let output = {
        match initial_parse(matrix).finish() {
            Ok(("", initial)) => Matrix { name: name.to_string(), rows: initial.len(), columns: initial[0].len(), content: initial },
            Err(e) => panic!("Cannot parse matrix starting from {}", e.input),
            _ => panic!("This message should not be possible")
        }
    };

    output.content.iter()
        .map(|row| row.len())
        .all_equal().then(|| "").ok_or("")
        .expect(format!("Matrix '{name}' has unequal rows")
            .as_str());

    return output;

    fn row_parse(s: &str) -> IResult<&str, Vec<String>>{
        let(remaining, (row, _)) =
            ws(many_till(
                ws(alphanumeric1),
                alt((
                    ws(tag("|")),
                    ws(eof)
                ))
            ))(s)?;

        Ok((remaining, row.iter().map(|str| str.to_string()).collect()))
    }

    fn initial_parse(s: &str) -> IResult<&str, Vec<Vec<String>>> {
        let (remaining, (matrix, _)) =
            ws(many_till(ws(row_parse), ws(eof)))(s)?;

        Ok((remaining, matrix))
    }
}

#[macro_export]
macro_rules! matrix{
    ($name:ident, $matrix:expr) => {
        let $name = matrix_parse($matrix, stringify!($name));
    }
}

fn get_position(index: usize, height: usize, width: usize) -> (usize, usize) {
    let column = index % width;
    let row = index / width;
    if row >= height {panic!("Matrix index is too high")};
    (row, column)
}

fn matrix_entry_to_code(matrix: &Matrix, column: usize, row: usize) -> Code {
    let entry = &matrix.content[row][column];
    let name = &matrix.name;
    format!("{name}{row}{column} = {entry};")
}


impl Matrix {
    pub fn matrix_multiply(&self, a: InGame<Matrix>, b: InGame<Matrix>) {
        
    }
}











