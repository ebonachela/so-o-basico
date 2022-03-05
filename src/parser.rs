use crate::tokens::Token;
use std::collections::VecDeque;

#[derive(Copy, Clone, Debug)]
pub enum Node {
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

pub fn parse(mut tokens: VecDeque<Token>) -> Vec<Node> {
    let mut result: Vec<Node> = Vec::new();

    result.push(expression(&mut tokens));

    result
}

fn factor(tokens: &mut VecDeque<Token>) -> Node {
    let result = *tokens.front().unwrap();
    Node::Number(result)
}

fn term(tokens: &mut VecDeque<Token>) -> Node {
    return binary_operation(tokens, factor, &[Token::Multiplication, Token::Division]);
}

fn expression(tokens: &mut VecDeque<Token>) -> Node {
    return binary_operation(tokens, term, &[Token::Plus, Token::Minus]);
}

fn binary_operation(tokens: &mut VecDeque<Token>, f: fn(&mut VecDeque<Token>) -> Node, acc_list: &[Token]) -> Node {
    let mut left = f(tokens);
    let current_token = *tokens.front().unwrap();

    while matches!(current_token, acc_list) {
        let operation = *tokens.front().unwrap();
        tokens.pop_front();
        let right = f(tokens);

        let mut left_value = Token::Integer(1);
        let mut right_value = Token::Integer(1);


        match left {
            Node::Number(i) => left_value = i,
            _ => panic!("Error")
        }

        match right {
            Node::Number(i) => right_value = i,
            _ => panic!("Error")
        }
        
        left = Node::BinaryOperation {
            left: left_value, 
            operation, 
            right: right_value
        }
    }

    left
}