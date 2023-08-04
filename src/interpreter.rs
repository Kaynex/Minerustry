use crate::parser::Line;

fn read_label_line(line: Line) -> String {
    match line {
        Line::LabelLine{label} => "label:".to_string(),
        _ => panic!("This message should never appear")
    }
}

fn read_label_line(line: Line) -> String {
    match line {
        Line::LabelLine{label} => "label:".to_string(),
        _ => panic!("This message should never appear")
    }
}