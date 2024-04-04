use std::env;
use std::fs;

#[derive(Debug)]
enum Symbol {
    OpenParen,
    CloseParen,
    Period,
    Comma
}
#[derive(Debug)]
enum Keyword {

}
#[derive(Debug)]
enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
    Identifier(String),
    Comment(String),
    StringLiteral(String)
}

fn main() {
    let mut tokens: Vec<Token> = Vec::new();
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).unwrap();

    // TODO: Replace with Buffered Reader
    let contents = fs::read_to_string(filename).expect("File read failed");
    let mut current_token = String::new();
    for item in contents.chars() {
        if current_token.len() > 0 && current_token.chars().nth(0).unwrap() == '[' {
            current_token.push(item);
            if item == ']' {
                tokens.push(Token::Identifier(String::from(current_token)));
                current_token = String::new();
            }
        }
        match item {
            '(' => tokens.push(Token::Symbol(Symbol::OpenParen)),
            ')' => tokens.push(Token::Symbol(Symbol::CloseParen)),
            ',' => tokens.push(Token::Symbol(Symbol::Comma)),
            '.' => tokens.push(Token::Symbol(Symbol::Period))
        }
    }
    println!("Hello, world!");
}
