use crate::tokens::Token;

#[derive(Clone, Debug)]
pub enum Node {
    Number(Token),
    Operation(Token),
    BinaryOperation {
        left: Box<Node>,
        operation: Box<Node>,
        right: Box<Node>
    },
    UnaryOperation {
        operation: Box<Node>,
        right: Box<Node>
    },
    NoneType
}