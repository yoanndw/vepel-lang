use crate::ast::*;

#[derive(Debug)]
pub struct CharGrid {
    chars: Vec<char>,
    width: usize,
}

fn grid_width(input: &String) -> usize {
    let lines = input.lines();

    lines
        .map(|l| l.chars().filter(|&c| c != ' ').collect::<Vec<_>>().len())
        .max()
        .unwrap()
}

pub fn string_to_chars(s: &String) -> CharGrid {
    let mut chars = Vec::new();
    let width = grid_width(s);

    for (i, c) in s.char_indices() {
        let col = i % width;
        if c != '\n' {
            chars.push(c);
        // End of line but not reached width yet
        } else if col < width - 1 {
            for _ in col..(width - 1) {
                chars.push(' ');
            }
        }
    }

    CharGrid { chars, width }
}

pub fn print_node_pos(input: String) -> (usize, usize) {
    for (row, line) in input.lines().enumerate() {
        if let Some(col) = line.find('$') {
            return (row, col);
        }
    }

    (0, 0)
}
