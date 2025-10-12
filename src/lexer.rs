use std::fs;

pub enum Type {
    Int
}

pub enum Token {
    Let,
    Identifier(String),
    Type(Type),
    NumberLiteral(u64),
}

pub struct Lexer {
    pub file_to_lex: String
}

impl Lexer {
    pub fn lex(&self) -> Vec<Token> {
        let contents  = fs::read_to_string(&self.file_to_lex).expect("File does not exist!");
        let mut tokens: Vec<String> = Vec::new();
        let mut curr_token = String::new();
        for char in contents.chars() {
            match char {
                ' ' | ';' => {
                    tokens.push(curr_token.clone());
                    curr_token.clear();
                },
                'a'..='z' => curr_token.push(char),
                '0'..='9' => curr_token.push(char),
                _ => ()

            }
        }
        tokens.iter().map(Lexer::determine_token_from_string).collect()
    }
    fn only_digits(str: &str) -> bool {
        if str.is_empty() {
            return false;
        }
        for char in str.chars() {
            if !char.is_ascii_digit() {
                return false;
            }
        }
        true
    }
    fn determine_token_from_string(str: &String) -> Token {
        match str.as_str() {
            "let" => Token::Let,
            "int" => Token::Type(Type::Int),
            num_lit if Lexer::only_digits(num_lit) => Token::NumberLiteral(num_lit.parse::<u64>().unwrap()),
            id => Token::Identifier(String::from(id)),
        }
    }
}

