use crate::tokens::Token;
use std::collections::VecDeque;

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
        rigth: Box<Node>
    },
    NoneType
}

pub fn parse(mut tokens: VecDeque<Token>) -> Vec<Node> {
    let mut result: Vec<Node> = Vec::new();

    let node_result: Node = expression(&mut tokens);

    match node_result {
        Node::NoneType => (),
        _ => result.push(node_result)
    }

    result
}

fn factor(tokens: &mut VecDeque<Token>) -> Node {
    if tokens.is_empty() {
        return Node::NoneType;
    }

    let result = *tokens.front().unwrap();

    tokens.pop_front();

    Node::Number(result)
}

fn term(tokens: &mut VecDeque<Token>) -> Node {
    if tokens.is_empty() {
        return Node::NoneType;
    }

    binary_operation(tokens, factor, &[Token::Multiplication, Token::Division])
}

fn expression(tokens: &mut VecDeque<Token>) -> Node {
    if tokens.is_empty() {
        return Node::NoneType;
    }

    binary_operation(tokens, term, &[Token::Plus, Token::Minus])
}

fn binary_operation(tokens: &mut VecDeque<Token>, f: fn(&mut VecDeque<Token>) -> Node, acc_list: &[Token]) -> Node {    
    let mut left = f(tokens);

    if tokens.is_empty() {
        return left;
    }

    while !tokens.is_empty() && acc_list.contains(tokens.front().unwrap()) {
        if tokens.is_empty() {
            return Node::NoneType;
        }

        let operation = *tokens.front().unwrap();
        tokens.pop_front();
        let right = f(tokens);
       
        left = Node::BinaryOperation {
            left: Box::new(left), 
            operation: Box::new(Node::Operation(operation)), 
            right: Box::new(right)
        }
    }

    left
}