use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::{alphanumeric1, line_ending, multispace1};
use nom::combinator::{eof, value};
use nom::error::ParseError;
use nom::{Finish, IResult};
use nom::multi::{many0, many_till};
use nom::sequence::{preceded, tuple};
use crate::parser::ExpressionKey::*;

#[test]
fn test() {
    dbg!(encode("\
    jump hya 5<3;\
    x = 3;\
    vault1.enabled = true;
    label hya;".to_string()));
}

#[derive(Debug, Clone)]
pub enum ExpressionKey {
    Free(String),
    Plus,
    Minus,
    Equal ,
    NotEq ,
    LessThan ,
    LessOrEq ,
    GreatThat ,
    GreatOrEq ,
    StrictEq ,
    Always
}

#[derive(Debug, Clone)]
pub enum Line {
    OpLine {assign: String, expression: Vec<ExpressionKey>},
    LabelLine {label: String},
    JumpLine {label: String, expression: Vec<ExpressionKey>},
    ControlLine {object: String, property: String, expression: Vec<ExpressionKey>}
}

pub type Code = String;

//wrapper for removing whitespace/line endings before tokens
pub(crate) fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
    where
        F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    preceded(
        many0(alt((multispace1, line_ending))),
        inner
    )
}

fn match_free (s: &str) -> IResult<&str, ExpressionKey> {
    let (remaining, accepted) = ws(alphanumeric1)(s)?;

    Ok((remaining, Free(accepted.to_string())))
}

fn expression_keyword(s: &str) -> IResult<&str, ExpressionKey> {
    alt((
        ws(value(Plus, tag("+"))),
        ws(value(Minus, tag("-"))),
        ws(value(StrictEq, tag("==="))),
        ws(value(Equal, tag("=="))),
        ws(value(LessThan, tag("<"))),
        ws(match_free)
        ))(s)
}

fn parse_operation_line (s: &str) -> IResult<&str, Line> {
    let (remaining, (Free(assign), _, (expression, _))) =
        tuple((
            ws(match_free),
            ws(tag("=")),
            ws(many_till(ws(expression_keyword), ws(tag(";"))))
        ))(s)? else {unreachable!()};

    Ok((remaining, Line::OpLine{assign, expression}))
}

fn parse_label_line (s: &str) -> IResult<&str, Line> {
    let (remaining, (_, label, _)) =
        tuple((
            ws(tag_no_case("label")),
            ws(alphanumeric1), //parse the string for the label
            ws(alt((tag(":"),tag(";"))))
        ))(s)?;

    let label = label.to_string();
    Ok((remaining, Line::LabelLine{label}))
}

fn parse_jump_line (s: &str) -> IResult<&str, Line> {
    let (remaining, (_, label, (expression, _))) =
        tuple((
            ws(tag_no_case("jump")),
            ws(alphanumeric1), //parse the string for the label
            ws(many_till(ws(expression_keyword), ws(tag(";")))),
        ))(s)?;

    let label = label.to_string();
    Ok((remaining, Line::JumpLine{label, expression}))
}

fn parse_control_line (s: &str) -> IResult<&str, Line> {
    let (remaining, (object, _, property, _, (expression, _))) =
        tuple((
            ws(alphanumeric1),
            ws(tag(".")),
            ws(alphanumeric1),
            ws(tag("=")),
            ws(many_till(ws(expression_keyword), ws(tag(";")))),
        ))(s)?;

    let object = object.to_string();
    let property = property.to_string();
    Ok((remaining, Line::ControlLine{object, property, expression}))
}

fn parse_any_line (s: &str) -> IResult<&str, Line> {
    alt((
        parse_operation_line,
        parse_control_line,
        parse_jump_line,
        parse_label_line
        ))(s)
}

fn parse_program (s: &str) -> IResult<&str, Vec<Line>> {
    let (_, (program, _)) =
    many_till(parse_any_line, ws(eof))(s)?;

    Ok(("", program))
}

pub fn encode (s: String) -> Result<Vec<Line>, String> {
    match parse_program(s.as_str()).finish() {
        Ok(("", vec)) => Ok(vec),
        Err(e) => Err(format!("Cannot parse starting from {}", e.input)),
        _ => unreachable!()
    }
}
