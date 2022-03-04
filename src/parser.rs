use crate::tokens::Token;

enum Node {
    Number(Token),
    BinaryOperation {
        left: Token,
        operation: Token,
        right: Token
    },
    UnaryOperation {
        operation: Token,
        rigth: Token
    }
}