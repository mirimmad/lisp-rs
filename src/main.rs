use std::collections::HashMap;
use linefeed::{interface, Interface, ReadResult};

mod eval;
mod lexer;
mod parser;

use parser::Object;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /* let t = lexer::Token::Integer(1);
    //let p = "((* 1 5) (define a 1))";
    //let p = "(if (= 5 5) (+ 1 1) (* 5 2))";
    let p = "((define a 1)(+ a 1))";
    //let tokens = lexer::tokenize(p).unwrap();
    //for token in tokens {
    // print!("{token} ");
    //}
    println!("");
    let parse_obj = parser::parse(p).unwrap();
    println!("{parse_obj}");
    let mut dict = HashMap::new();
    let result = eval::eval_obj(&parse_obj, &mut dict).unwrap();
    println!("{result}") */
    // println!(" Hello, world! {t}");

    let mut parser_result;
    let mut eval_result;
    let mut dict:HashMap<String, Object> = HashMap::new();
    let  reader = Interface::new("lisp reader")?;
    reader.set_prompt("λ: ")?;

    while let ReadResult::Input(input) = reader.read_line()? {
        reader.add_history(input.clone());
        parser_result = parser::parse(input.as_str());
        if parser_result.is_err() {
            println!("{:?}", parser_result);
            continue;
        }

        eval_result = eval::eval_obj(&(parser_result.unwrap()), &mut dict);
        if eval_result.is_err() {
            println!("{:?}", eval_result);
            continue;
        } else {
            match eval_result.unwrap() {
                Object::Integer(n) => println!("{n}"),
                Object::Bool(b) => println!("{b}"),
                _  => continue,
            }
        }
    }

    Ok(())
}
