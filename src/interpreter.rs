// Basic interpreter
use crate::nodes::Node;
use crate::tokens::Token;
use crate::macros;

pub fn interpret(init_node: Node) {
    let result: Node = visit_node(init_node);

    println!("{:?}", macros::get_variant!(macros::get_variant!(result, Node::Number).unwrap(), Token::Integer).unwrap());
}

fn visit_node(node: Node) -> Node {
    let mut result: Node = Node::NoneType;

    match node {
        Node::BinaryOperation {left: _, operation: _, right: _} => {
            result = visit_binop_node(node);
        },
        Node::UnaryOperation {operation: _, right: _} => {
            ()
        },
        Node::Number(_) => result = node,
        _ => ()
    }

    result
}

fn visit_binop_node(node: Node) -> Node {
    let mut result: Node = Node::NoneType;

    match node {
        Node::BinaryOperation {left, operation, right} => {
            let left_node: Node = visit_node(*left);
            let operation_node: Node = *operation;
            let right_node: Node = visit_node(*right);

            match operation_node {
                Node::Operation(operation_value) => {
                    match operation_value {
                        Token::Plus => result = left_node + right_node,
                        Token::Minus => result = left_node - right_node,
                        Token::Multiplication => result = left_node * right_node,
                        Token::Division => result = left_node / right_node,
                        _ => ()
                    }
                },
                _ => ()
            }
        },
        _ => ()
    }

    result
}