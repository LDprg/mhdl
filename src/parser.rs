use crate::prelude::*;

#[derive(Clone, Debug)]
pub enum Expr<'src> {
    Output(&'src str),
    Input(&'src str),
    Signal(&'src str),
    And(Box<Spanned<Self>>, Box<Spanned<Self>>),
}

pub fn parser<'src, I, M>(
    make_input: M,
) -> impl Parser<'src, I, Token<'src>, extra::Err<Rich<'src, Token<'src>>>>
where
    I: BorrowInput<'src, Token = Token<'src>, Span = SimpleSpan>,
    // Because this function is generic over the input type, we need the caller to tell us how to create a new input,
    // `I`, from a nested token tree. This function serves that purpose.
    M: Fn(SimpleSpan, &'src [Spanned<Token<'src>>]) -> I + Clone + 'src,
{
    recursive(|expr| {
        let scope = choice((
            just(Token::InputScope),
            just(Token::OutputScope),
        ));

        scope
    })
}
