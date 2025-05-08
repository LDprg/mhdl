use crate::prelude::*;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Variable<'src> {
    name: &'src str,
    typ: VariableType,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum Scope<'src> {
    Input(Vec<Variable<'src>>),
    Output(Vec<Variable<'src>>),
    Signal(Vec<Variable<'src>>),
    Logic(Vec<Token<'src>>),
}

impl fmt::Display for Scope<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Scope::Input(_) => write!(f, "input"),
            Scope::Output(_) => write!(f, "output"),
            Scope::Signal(_) => write!(f, "signal"),
            Scope::Logic(_) => write!(f, "logic"),
        }
    }
}

pub fn scopes_parser<'src, I>()
-> impl Parser<'src, I, Vec<Scope<'src>>, extra::Err<Rich<'src, Token<'src>, Span>>> + Clone
where
    I: ValueInput<'src, Token = Token<'src>, Span = Span>,
{
    choice((
        just(Token::InputScope)
            .then(
                vars_parser()
                    .separated_by(just(Token::Ctrl(',')))
                    .allow_trailing()
                    .collect::<Vec<_>>()
                    .delimited_by(just(Token::Ctrl('{')), just(Token::Ctrl('}'))),
            )
            .map(|(_, data)| Scope::Input(data)),
        just(Token::OutputScope)
            .then(
                vars_parser()
                    .separated_by(just(Token::Ctrl(',')))
                    .allow_trailing()
                    .collect::<Vec<_>>()
                    .delimited_by(just(Token::Ctrl('{')), just(Token::Ctrl('}'))),
            )
            .map(|(_, data)| Scope::Output(data)),
        just(Token::SignalScope)
            .then(
                vars_parser()
                    .separated_by(just(Token::Ctrl(',')))
                    .allow_trailing()
                    .collect::<Vec<_>>()
                    .delimited_by(just(Token::Ctrl('{')), just(Token::Ctrl('}'))),
            )
            .map(|(_, data)| Scope::Signal(data)),
        just(Token::LogicScope)
            .then(
                none_of(Token::Ctrl('}'))
                    .repeated()
                    .collect::<Vec<_>>()
                    .delimited_by(just(Token::Ctrl('{')), just(Token::Ctrl('}'))),
            )
            .map(|(_, data)| Scope::Logic(data)),
    ))
    .labelled("Scope")
    .repeated()
    .collect::<Vec<_>>()
}

pub fn vars_parser<'src, I>()
-> impl Parser<'src, I, Variable<'src>, extra::Err<Rich<'src, Token<'src>, Span>>> + Clone
where
    I: ValueInput<'src, Token = Token<'src>, Span = Span>,
{
    let ident = select! {Token::Ident(s) => s}.labelled("Identifier");
    let typ = select! {Token::VariableType(v) => v}.labelled("Type");

    ident
        .then_ignore(just(Token::Ctrl(':')))
        .then(typ)
        .map(|(s, t)| Variable { name: s, typ: t })
        .labelled("Variable")
}
