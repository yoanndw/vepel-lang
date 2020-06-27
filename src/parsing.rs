use crate::ast::*;

#[derive(Debug)]
pub struct CharGrid {
    chars: Vec<char>,
    width: usize,
}

fn grid_width(input: String) -> usize {
    let lines = input.lines();

    lines
        .map(|l| l.chars().filter(|&c| c != ' ').collect::<Vec<_>>().len())
        .max()
        .unwrap()
}

pub fn string_to_chars(s: String) -> CharGrid {
    let mut chars = Vec::new();
    let mut found_width = false;
    let mut width = 0;

    for (i, c) in s.char_indices() {
        //println!("char {}/{}", i, width);
        /*
        if c == '\n' && !found_width {
            width = i;
            found_width = true;
        }

        let col = i / width;
        if c != '\n' {
            chars.push(c);
        }
        // New line too early
        else if found_width && col < (width - 1) {
            let spaces = " ".repeat(col - i);
            chars.extend(spaces.chars());
        }*/
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
