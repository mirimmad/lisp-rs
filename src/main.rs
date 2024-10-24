mod lexer;


fn main() {
    let t = lexer::Token::Integer(1);
    let p = "(+ 1 1 define hello)";
    let tokens = lexer::tokenize(p).unwrap();
    for token in tokens {
        print!("{token} ");
    }
    println!(" Hello, world! {t}");
}
