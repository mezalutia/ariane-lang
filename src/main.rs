mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = args.get(1).expect("No file path given!");
    let lexer = lexer::Lexer { file_to_lex: file_path.to_owned() };
    let tokens = lexer.lex();
    for token in tokens.iter() {
        match token {
            lexer::Token::Keyword(keyword) => print!("[Keyword:{:?}]", keyword),
            lexer::Token::Identifier(name) => print!("[id:{name}]"),
            lexer::Token::Colon => print!(":"),
            lexer::Token::Type(type_name) => print!("[Type:{:?}]", type_name),
            lexer::Token::Assignment => print!("="),
            lexer::Token::NumberLiteral(n) => print!("{n}"),
            lexer::Token::Semicolon => println!(";"),
            _ => (),
        }
    }
    let parser = parser::Parser { tokens };
    let ast = parser.parse();
}
