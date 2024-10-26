use crate::parser::Object;
use std::collections::HashMap;

fn eval_binary(list: &Vec<Object>, dict: &mut HashMap<String, Object>) -> Result<Object, String> {
    if list.len() != 3 {
        return Err(String::from("Binary operator must have two operands"));
    }
    let op_obj = &list[0];
    let lhs_obj = eval_obj(&list[1], dict)?;
    let rhs_obj = eval_obj(&list[2], dict)?;

    let op = match op_obj {
        Object::Symbol(sym) => sym.as_str(),
        _ => return Err(String::from("Invalid operator")),
    };

    let lhs = match lhs_obj {
        Object::Integer(n) => n,
        _ => return Err(String::from("Operands must be numeric")),
    };

    let rhs = match rhs_obj {
        Object::Integer(n) => n,
        _ => return Err(String::from("Operands must be numeric")),
    };

    match op {
        "+" => Ok(Object::Integer(lhs + rhs)),
        "-" => Ok(Object::Integer(lhs - rhs)),
        "*" => Ok(Object::Integer(lhs * rhs)),
        "/" => Ok(Object::Integer(lhs / rhs)),
        "=" => Ok(Object::Bool(lhs == rhs)),
        ">" => Ok(Object::Bool(lhs > rhs)),
        "<" => Ok(Object::Bool(lhs < rhs)),
        "!=" => Ok(Object::Bool(lhs != rhs)),
        _ => return Err(String::from("Unkown operator")),
    }

    // Ok(Object::Integer(1))
}

fn eval_define(list: &Vec<Object>, dict: &mut HashMap<String, Object>) -> Result<Object, String> {
    if list.len() != 3 {
        return Err(String::from("'define' must have two arguments"));
    }

    let dst_obj = &list[1];
    let dst = match dst_obj {
        Object::Symbol(sym) => sym.clone(),
        _ => return Err(String::from("invalid define")),
    };
    let src_obj = eval_obj(&list[2], dict)?;
    //let test = dst.clone();
    dict.insert(dst, src_obj);

    //println!("define {} at {}", dict.get(&test).unwrap(), &test);
    Ok(Object::Void)
}

fn eval_if(list: &Vec<Object>, dict: &mut HashMap<String, Object>) -> Result<Object, String> {
    if list.len() != 4 {
        return Err(String::from("Invalid 'if' statement"));
    }
    let predicate_obj = eval_obj(&list[1], dict).unwrap();
    let predicate = match predicate_obj {
        Object::Bool(b) => b,
        _ => return Err(String::from("Condition must be boolean")),
    };
    if predicate {
        return eval_obj(&list[2], dict);
    } else {
        return eval_obj(&list[3], dict);
    }
}

fn eval_symbol(sym: &String, dict: &mut HashMap<String, Object>) -> Result<Object, String> {
    let value = dict.get(sym);
    match value {
        Some(_) => return Ok(value.unwrap().clone()),
        None => return Err(String::from("Unbound symbol")),
    }
}

fn eval_list(list: &Vec<Object>, dict: &mut HashMap<String, Object>) -> Result<Object, String> {
    let head = &list[0];

    match head {
        Object::Symbol(sym) => match sym.as_str() {
            "+" | "-" | "*" | "/" | "=" | "!=" | ">" | "<" => return eval_binary(list, dict),

            "define" => eval_define(list, dict),
            "if" => eval_if(list, dict),
            _ => panic!("Not Supported {}", sym.as_str()),
        },
        _ => {
            //panic!("Not supported {head}")
            let mut new_list = Vec::new();
            for obj in list {
                let ret = eval_obj(obj, dict)?;
                match ret {
                    Object::Void => {}
                    _ => new_list.push(ret),
                }
            }
            Ok(Object::List(new_list))
        }
    }
}

pub fn eval_obj(obj: &Object, dict: &mut HashMap<String, Object>) -> Result<Object, String> {
    /* dict.insert(String::from("a"), Object::Integer(1));
    Ok(Object::Integer(1)) */
    match obj {
        Object::Void => Ok(obj.clone()),
        Object::Integer(_) => Ok(obj.clone()),
        Object::Bool(_) => Ok(obj.clone()),
        Object::List(list) => eval_list(list, dict),
        Object::Symbol(sym) => eval_symbol(sym, dict),
        _ => panic!("What's this? {obj}"),
    }
}
