use crate::lexer::{Token, Keyword, ArianeType};
use std::iter::Peekable;

pub enum Term {
    Number(u64),
    Mult(Box<Term>, Box<Term>)
}

pub enum Expression {
    Program,
    Term(Term),
    Plus(Term, Term),
    VariableDeclaration(String, ArianeType, Box<Expression>)
}

pub struct ParseNode {
    value: Expression,
    children: Vec<ParseNode>,
}

pub struct Parser {
    pub tokens: Vec<Token>
}

impl Parser {
    pub fn parse(&self) -> ParseNode {
        let mut root = ParseNode {
            value: Expression::Program,
            children: Vec::new(),
        };
        let tokens = &mut self.tokens.iter().peekable();
        let token_option = tokens.next();
        while let Some(curr_token) = token_option {
            match curr_token {
                Token::Keyword(keyword) => match keyword {
                    Keyword::Let => root.children.push(ParseNode {value: Parser::parse_var_decl(tokens), children: Vec::new()}),
                },
                _ => todo!(),
            }
            tokens.next();
        }
        root
    }
    fn parse_var_decl<'a, I: Iterator<Item = &'a Token>>(tokens: &mut Peekable<I>) -> Expression {
        let Some(Token::Identifier(id_name)) = tokens.next() else {panic!("Expected identifier name.")};
        match tokens.next() {
            Some(Token::Colon) => (),
            _ => panic!("Expected colon.")
        }
        let Some(Token::Type(ariane_type)) = tokens.next() else {panic!("Expected type name.")};
        match tokens.next() {
            Some(Token::Assignment) => (),
            _ => panic!("Expected assignment.")
        }
        let Some(Token::NumberLiteral(n)) = tokens.next() else {panic!("Expected number literal, for now.")};
        match tokens.next() {
            Some(Token::Semicolon) => (),
            _ => panic!("Expected semicolon.")
        }
        let expr = Box::new(Expression::Term(Term::Number(*n)));
        Expression::VariableDeclaration(
            id_name.to_owned(), 
            ariane_type.to_owned(),
            expr
        )
    }
}
