use crate::prelude::*;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum Scope<'src> {
    Input(Vec<Token<'src>>),
    Output(Vec<Token<'src>>),
    Signal(Vec<Token<'src>>),
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
                none_of(Token::Ctrl('}'))
                    .repeated()
                    .collect::<Vec<_>>()
                    .delimited_by(just(Token::Ctrl('{')), just(Token::Ctrl('}'))),
            )
            .map(|(_, data)| Scope::Input(data)),
        just(Token::OutputScope)
            .then(
                none_of(Token::Ctrl('}'))
                    .repeated()
                    .collect::<Vec<_>>()
                    .delimited_by(just(Token::Ctrl('{')), just(Token::Ctrl('}'))),
            )
            .map(|(_, data)| Scope::Output(data)),
        just(Token::SignalScope)
            .then(
                none_of(Token::Ctrl('}'))
                    .repeated()
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
    .repeated()
    .collect::<Vec<_>>()
}
