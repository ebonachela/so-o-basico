use text_io::read;
use std::io;
use std::io::Write;
use std::collections::VecDeque;

mod tokens;
mod lexer;
mod parser;

fn main() {
    loop {
        print!("only the basic > ");
        io::stdout().flush().unwrap();

        let mut input_text: String = read!("{}\n");
        
        if input_text.ends_with('\r') {
            input_text.pop();
        }

        if input_text == "break" {
            println!("Stoping...");
            break;
        }

        let tokens: VecDeque<tokens::Token> = VecDeque::from(lexer::create_token(input_text));

        let parser_tokens: Vec<parser::Node> = parser::parse(tokens);

        println!("{:?}", parser_tokens);
    }
}
