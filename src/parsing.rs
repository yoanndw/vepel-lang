use crate::ast::*;

pub fn print_node_pos(input: String) -> (usize, usize) {
    for (row, line) in input.lines().enumerate() {
        if let Some(col) = line.find('$') {
            return (row, col);
        }
    }

    (0, 0)
}
