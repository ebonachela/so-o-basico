#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Token {
    Integer(i32),
    Plus,
    Minus,
    Multiplication,
    Division,
    LeftParenthesis,
    RigthParenthesis
}