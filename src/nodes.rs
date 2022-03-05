use crate::tokens::Token;
use std::ops;

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

impl ops::Add<Node> for Node {
    type Output = Node;

    fn add(self, other: Node) -> Node {
        let mut value1_t: Token = Token::Integer(0);
        let mut value2_t: Token = Token::Integer(0);

        let mut value1: i32 = 0;
        let mut value2: i32 = 0;

        match self {
            Node::Number(v1) => value1_t = v1,
            _ => ()
        }

        match other {
            Node::Number(v1) => value2_t = v1,
            _ => ()
        }

        match value1_t {
            Token::Integer(v1) => value1 = v1,
            _ => ()
        }

        match value2_t {
            Token::Integer(v1) => value2 = v1,
            _ => ()
        }

        Node::Number(Token::Integer(value1 + value2))
    }
}

impl ops::Sub<Node> for Node {
    type Output = Node;

    fn sub(self, other: Node) -> Node {
        let mut value1_t: Token = Token::Integer(0);
        let mut value2_t: Token = Token::Integer(0);

        let mut value1: i32 = 0;
        let mut value2: i32 = 0;

        match self {
            Node::Number(v1) => value1_t = v1,
            _ => ()
        }

        match other {
            Node::Number(v1) => value2_t = v1,
            _ => ()
        }

        match value1_t {
            Token::Integer(v1) => value1 = v1,
            _ => ()
        }

        match value2_t {
            Token::Integer(v1) => value2 = v1,
            _ => ()
        }

        Node::Number(Token::Integer(value1 - value2))
    }
}

impl ops::Mul<Node> for Node {
    type Output = Node;

    fn mul(self, other: Node) -> Node {
        let mut value1_t: Token = Token::Integer(0);
        let mut value2_t: Token = Token::Integer(0);

        let mut value1: i32 = 0;
        let mut value2: i32 = 0;

        match self {
            Node::Number(v1) => value1_t = v1,
            _ => ()
        }

        match other {
            Node::Number(v1) => value2_t = v1,
            _ => ()
        }

        match value1_t {
            Token::Integer(v1) => value1 = v1,
            _ => ()
        }

        match value2_t {
            Token::Integer(v1) => value2 = v1,
            _ => ()
        }

        Node::Number(Token::Integer(value1 * value2))
    }
}

impl ops::Div<Node> for Node {
    type Output = Node;

    fn div(self, other: Node) -> Node {
        let mut value1_t: Token = Token::Integer(0);
        let mut value2_t: Token = Token::Integer(0);

        let mut value1: i32 = 0;
        let mut value2: i32 = 0;

        match self {
            Node::Number(v1) => value1_t = v1,
            _ => ()
        }

        match other {
            Node::Number(v1) => value2_t = v1,
            _ => ()
        }

        match value1_t {
            Token::Integer(v1) => value1 = v1,
            _ => ()
        }

        match value2_t {
            Token::Integer(v1) => value2 = v1,
            _ => ()
        }

        Node::Number(Token::Integer(value1 / value2))
    }
}