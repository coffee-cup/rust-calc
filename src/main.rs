use std::io;
use std::io::Write;

mod lexer;
mod parser;

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().expect("Error flushing stdout");

        match read_line().trim() {
            "" => (),
            line => {
                let tokens = lexer::lex(&line.to_owned());
                println!("{:?}", tokens);

                let ast = parser::parse(&line.to_owned());
                println!("{:?}", ast);
            }
        }
    }
}

fn read_line() -> String {
    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
        Ok(_) => buf,
        Err(e) => {
            println!("{:?}", e);
            "".to_string()
        }
    }
}
