pub enum Instruction {
    Print(Node),
}

pub enum Node {
    Int(i32),
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
}

impl Node {
    pub fn eval(&self) -> i32 {
        match self {
            Node::Int(i) => *i,
            Node::Add(n1, n2) => n1.eval() + n2.eval(),
            Node::Sub(n1, n2) => n1.eval() - n2.eval(),
            Node::Mul(n1, n2) => n1.eval() * n2.eval(),
            Node::Div(n1, n2) => n1.eval() / n2.eval(),
        }
    }
}

impl Instruction {
    pub fn eval(&self) {
        match self {
            Instruction::Print(n) => println!("{}", n.eval()),
        }
    }
}
