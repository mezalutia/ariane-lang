mod lexer;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = args.get(1).expect("No file path given!");
    let lexer = lexer::Lexer {
        file_to_lex: file_path.to_owned(),
    };
    let tokens = lexer.lex();
    for token in tokens.iter() {
        match token {
            lexer::Token::Let => print!("[let]"),
            lexer::Token::Identifier(name) => print!("[{name}]"),
            lexer::Token::Colon => print!(":"),
            lexer::Token::Type(type_name) => print!("{:?}", type_name),
            lexer::Token::Assignment => print!("="),
            lexer::Token::NumberLiteral(n) => print!("{n}"),
            lexer::Token::Semicolon => println!(";"),
            _ => (),
        }
    }
}
