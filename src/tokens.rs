#[derive(Copy, Clone, Debug)]
pub enum Token {
    Integer(i32),
    Plus,
    Minus,
    Multiplication,
    Division
}