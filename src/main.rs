mod ast;
mod parsing;

use ast::*;
use parsing::*;

fn main() {
    // print(5 + 3)
    // let code = Instruction::Print(Node::Add(Box::new(Node::Int(5)), Box::new(Node::Int(3))));

    // code.eval();

    let input = String::from(
        r"5--------->+------->$
           ^
3----------|",
    );
    println!("{:?}", print_node_pos(input));
}
