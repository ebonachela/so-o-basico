use text_io::read;
use std::io;
use std::io::Write;
use std::collections::VecDeque;

mod tokens;
mod lexer;
mod parser;
mod nodes;
mod interpreter;

fn main() {
    loop {
        print!("only the basic > ");
        io::stdout().flush().unwrap();

        let mut input_text: String = read!("{}\n");
        
        if input_text.ends_with('\r') {
            input_text.pop();
        }

        match input_text.as_str() {
            "break" | "br" => {
                println!("Stoping...");
                break;
            },
            "clear" | "cls" => {
                print!("\x1B[2J\x1B[1;1H");
                continue;
            }
            _ => ()
        }

        let tokens: VecDeque<tokens::Token> = VecDeque::from(lexer::create_token(input_text));

        //println!("Tokens: {:?}", tokens);

        let parser_token: nodes::Node = parser::parse(tokens);

        //println!("Parser: {:?}", parser_token);

        interpreter::interpret(parser_token);
    }
}
