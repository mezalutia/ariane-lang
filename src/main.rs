mod lexer;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = args.get(1).expect("No file path given!");
    let lexer = lexer::Lexer {
        file_to_lex: file_path.to_string(),
    };
    let tokens = lexer.lex();
    for token in tokens.iter() {
        match token {
            lexer::Token::Let => println!("[let]"),
            lexer::Token::Identifier(name) => println!("[{name}]"),
            lexer::Token::Type(_) => println!("[TYPE]"),
            lexer::Token::NumberLiteral(n) => println!("{n}"),
        }
    }
}
