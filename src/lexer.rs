use std::fmt;
use std::error::Error;



#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Integer(i64),
    Symbol(String),
    LParen,
    RParen,
}
#[derive(Debug)]
pub struct LexerError {
    ch: char,
}

impl Error for LexerError {}


impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unexpected {}", self.ch)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Integer(n) => write!(f, "{n}"), 
            Token::Symbol(s) => write!(f, "{s}"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
        }
    }
}

pub fn tokenize(program: &str) -> Result<Vec<Token>, LexerError> {
    let program = program.replace("(", " ( ").replace(")", " ) ");
    let words = program.split_whitespace();
    let mut tokens: Vec<Token>= Vec::new();
    for word in words {
        match word {
            "(" => tokens.push(Token::LParen),
            ")" => tokens.push(Token::RParen),
            _ => {
                let i = word.parse::<i64>();
                match i {
                    Ok(num) => tokens.push(Token::Integer(num)),
                    Err(_) => tokens.push(Token::Symbol(word.to_string())),
                }
            }
        }
    }
    Ok(tokens)
}