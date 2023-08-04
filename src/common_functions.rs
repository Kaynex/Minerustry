use std::{thread, time};
use std::time::*;
use crate::logic::*;
use crate::parser::{Code, encode};

#[test]
fn test() {
    dbg!(encode(whilem("5 < 3", "x = 1;")));
}

pub fn generate_label() -> String {
    thread::sleep(time::Duration::from_micros(2));
    let label = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros();
    let label = label % 100000000000;
    bs62::encode_num(&label)
}

pub fn ifm(condition: &str, code: &str) -> Code {
    let label = generate_label();
    format!("\
        jump {label} {condition} === false;
        {code}
        {label}:
    ")
}

pub fn whilem(condition: &str, code: &str) -> Code {
    let label1 = generate_label();
    let label2 = generate_label();
    format!("\
        {label1}:
        jump {label2} {condition} === false;
        {code}
        jump {label1};
        {label2}:
    ")
}



















