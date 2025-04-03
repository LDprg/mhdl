use std::{env, fmt, fs};

use chumsky::prelude::*;

pub type Span = SimpleSpan;
pub type Spanned<T> = (T, Span);

#[derive(Clone, Debug)]
enum Token<'a> {
    // Values
    Bool(bool),
    Num(i64),

    // Tokens
    Ident(&'a str),
    Op(&'a str),
    Ctrl(char),

    // Types
    BoolType,
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // Values
            Token::Bool(x) => write!(f, "{}", x),
            Token::Num(n) => write!(f, "{}", n),
            // Tokens
            Token::Ident(s) => write!(f, "{}", s),
            Token::Op(s) => write!(f, "{}", s),
            Token::Ctrl(c) => write!(f, "{}", c),
            // Types
            Token::BoolType => write!(f, "bool"),
        }
    }
}

fn lexer<'a>() -> impl Parser<'a, &'a str, Vec<Spanned<Token<'a>>>, extra::Err<Rich<'a, char, Span>>>
{
    let num = text::int(10)
        .to_slice()
        .from_str()
        .unwrapped()
        .map(Token::Num);

    let op = one_of("&|!=")
        .repeated()
        .at_least(1)
        .to_slice()
        .map(Token::Op);
    let ctrl = one_of("(){}[];,:").map(Token::Ctrl);

    let ident = text::ascii::ident().map(|ident: &str| match ident {
        "bool" => Token::BoolType,
        "true" => Token::Bool(true),
        "false" => Token::Bool(false),
        _ => Token::Ident(ident),
    });

    let token = num.or(op).or(ctrl).or(ident);

    let comment = just("//")
        .then(any().and_is(just('\n').not()).repeated())
        .padded();

    token
        .map_with(|tok, e| (tok, e.span()))
        .padded_by(comment.repeated())
        .padded()
        // If we encounter an error, skip and attempt to lex the next character as a token instead
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}

fn main() {
    let path = env::args()
        .nth(1)
        .or_else(|| Some("./test/simple.mhdl".to_string()))
        .expect("No file path");
    let src = fs::read_to_string(path).expect("Failed to read file");

    match lexer().parse(src.trim()).into_result() {
        Ok(ast) => ast.into_iter().for_each(|data| println!("{:?}", data)),
        Err(errs) => errs.into_iter().for_each(|e| println!("{:?}", e)),
    };
}
