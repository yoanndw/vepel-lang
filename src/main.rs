mod ast;

use ast::*;

fn main() {
    // print(5 + 3)
    let code = Instruction::Print(Node::Add(Box::new(Node::Int(5)), Box::new(Node::Int(3))));

    code.eval();
}
