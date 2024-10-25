use std::fmt;

use crate::lexer::tokenize;
use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Object {
    Void,
    Integer(i64),
    Bool(bool),
    Symbol(String),
    List(Vec<Object>),
    Lambda(Vec<String>, Vec<Object>),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Void => write!(f, ""),
            Object::Integer(i) => write!(f, "{i}"),
            Object::Bool(b) => write!(f, "{b}"),
            Object::Symbol(sym) => write!(f, "{sym}"),
            Object::List(list) => {
                write!(f, "List (")?;
                for item in list.iter() {
                    write!(f, " {item} ")?;
                }
                write!(f, ") ")
            }
            Object::Lambda(params, body) => {
                write!(f, "Lambda ( ")?;
                for param in params.iter() {
                    write!(f, " {param} ")?;
                }
                write!(f, ")")?;
                write!(f, "body (")?;
                for obj in body.iter() {
                    write!(f, " {obj} ")?;
                }
                write!(f, ")")
            }

        }
    }
}   

pub fn parse(program: &str) -> Result<Object, String> {
    let tokens = tokenize(program);
    if tokens.is_err() {
        return Err(String::from("Lexer Error"));
    }
    let mut rev_tokens = tokens.unwrap().into_iter().rev().collect::<Vec<Token>>();
    parse_list(&mut rev_tokens)


}

fn parse_list(tokens: &mut Vec<Token>) -> Result<Object, String> {
    let  token = tokens.pop();
    if token != Some(Token::LParen) {
        return Err(String::from("Expected '(' "));
    }

    let mut objs:Vec<Object> = Vec::new();
    while !tokens.is_empty() {
        let token = tokens.pop();
        if token == None {
            return Err(String::from("Reached unexpected end."));
        }
        let t  = token.unwrap();
        match t {
            Token::Integer(n) => objs.push(Object::Integer(n)),
            Token::Symbol(sym) => objs.push(Object::Symbol(sym)),
            Token::RParen => {
                return Ok(Object::List(objs));
            }
            Token::LParen => {
                tokens.push(Token::LParen);
                let sub_list = parse_list(tokens)?;
                objs.push(sub_list);
            }

        }
    }

    return Ok(Object::List(objs))
}