pub enum Node {
    Int(i32),
    Add(Box<Node>, Box<Node>),
    Print(Box<Node>),
}
