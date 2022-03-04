use crate::tokens::Token;

pub fn create_token(text: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut build_integer: String = String::new();
    let mut should_build_integer: bool = false;

    for c in text.chars() {
        if c.is_numeric() {
            should_build_integer = true;
            build_integer.push(c);
            continue;
        }

        if !c.is_numeric() && should_build_integer {
            should_build_integer = false;
            tokens.push(Token::Integer(build_integer.parse::<i32>().unwrap()));
            build_integer = String::new();
        }

        match c {
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Multiplication),
            '/' => tokens.push(Token::Division),
            ' ' => should_build_integer = false,
            unknown => println!("Unknown token ({})", unknown)
        }
    }

    if should_build_integer {
        tokens.push(Token::Integer(build_integer.parse::<i32>().unwrap()));
    }

    tokens
}