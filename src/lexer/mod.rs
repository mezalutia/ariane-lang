use std::fs::File;
use std::io::BufReader;
use utf8_chars::BufReadCharsExt;

#[derive(Debug)]
pub enum Type {
    Int,
}

#[derive(Debug)]
pub enum Keyword {
    Let,
}

pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    Colon,
    Type(Type),
    Assignment,
    NumberLiteral(u64),
    Semicolon,
    EOF,
    Unknown,
}

pub struct Lexer {
    pub file_to_lex: String,
}

impl Lexer {
    pub fn lex(&self) -> Vec<Token> {
        let file = File::open(&self.file_to_lex).expect("File cannot be found!");
        let mut reader = BufReader::new(file);
        let mut chars = reader.chars().peekable();
        let mut str_buffer = String::new();
        let mut tokens_list: Vec<Token> = Vec::new();
        let mut curr_char_option = chars.next();
        while let Some(Ok(curr_char)) = curr_char_option {
            match curr_char {
                'a'..='z' | 'A'..='Z' | '0'..='9' => {
                    str_buffer.push(curr_char);
                    if let Some(Ok(next_char)) = chars.peek() && !next_char.is_alphanumeric() {
                        tokens_list.push(Lexer::determine_token(&str_buffer));
                        str_buffer.clear();
                    }
                }
                '=' => tokens_list.push(Token::Assignment),
                ';' => tokens_list.push(Token::Semicolon),
                ':' => tokens_list.push(Token::Colon),
                ' ' | '\n' => (),
                _ => tokens_list.push(Token::Unknown),
            }
            curr_char_option = chars.next();
        }
        tokens_list.push(Token::EOF);
        tokens_list
    }
    fn determine_token(id: &String) -> Token {
        match id.as_str() {
            "let" => Token::Keyword(Keyword::Let),
            "int" => Token::Type(Type::Int),
            num_lit if num_lit.parse::<u64>().is_ok() => {
                Token::NumberLiteral(num_lit.parse::<u64>().unwrap())
            }
            id => Token::Identifier(String::from(id)),
        }
    }
}
