use crate::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum VariableType {
    Bool,
}

impl fmt::Display for VariableType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VariableType::Bool => write!(f, "bool"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token<'a> {
    // Values
    Bool(bool),
    Num(i64),

    // Tokens
    Ident(&'a str),
    Op(&'a str),
    Ctrl(char),

    // Types
    VariableType(VariableType),

    // Scope
    OutputScope,
    InputScope,
    SignalScope,
    LogicScope,
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
            Token::VariableType(v) => write!(f, "{}", v),
            // Scopes
            Token::OutputScope => write!(f, "output"),
            Token::InputScope => write!(f, "input"),
            Token::SignalScope => write!(f, "scope"),
            Token::LogicScope => write!(f, "logic"),
        }
    }
}

pub fn lexer<'a>()
-> impl Parser<'a, &'a str, Vec<Spanned<Token<'a>>>, extra::Err<Rich<'a, char, Span>>> {
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
        // Values
        "true" => Token::Bool(true),
        "false" => Token::Bool(false),
        // Types
        "bool" => Token::VariableType(VariableType::Bool),
        // Scopes
        "output" => Token::OutputScope,
        "input" => Token::InputScope,
        "signal" => Token::SignalScope,
        "logic" => Token::LogicScope,
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
