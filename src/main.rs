use text_io::read;
use std::io;
use std::io::Write;

mod lexer;

fn main() {
    loop {
        print!("basic > ");
        io::stdout().flush().unwrap();

        let mut input_text: String = read!("{}\n");
        
        if input_text.ends_with('\r') 
        {
            input_text.pop();
        }

        if input_text == "break"
        {
            println!("Stoping...");
            break;
        }

        let tokens = lexer::create_token(input_text);

        println!("{:?}", tokens);
    }
}
