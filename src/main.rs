use std::{env, fs};

use chumsky::prelude::*;

#[derive(Clone, Debug)]
#[allow(dead_code)]
enum Scope {
    Input(String),
    Output(String),
    Test(String),
    Logic(String),
    Process(String),
}

fn parser<'a>() -> impl Parser<'a, &'a str, Vec<Scope>, extra::Err<Simple<'a, char>>> {
    recursive(|_| {
        let scope = none_of("{}")
            .repeated()
            .to_slice()
            .map(ToString::to_string)
            .delimited_by(just('{').padded().ignored(), just('}').padded().ignored());
        choice((
            just("input")
                .ignored()
                .then(scope)
                .map(|(_, data)| Scope::Input(data)),
            just("output")
                .ignored()
                .then(scope)
                .map(|(_, data)| Scope::Output(data)),
            just("test")
                .ignored()
                .then(scope)
                .map(|(_, data)| Scope::Test(data)),
            just("logic")
                .ignored()
                .then(scope)
                .map(|(_, data)| Scope::Logic(data)),
            just("process")
                .ignored()
                .then(scope)
                .map(|(_, data)| Scope::Process(data)),
        ))
        .repeated()
        .collect()
    })
}

fn main() {
    let path = env::args()
        .nth(1)
        .or_else(|| Some("./test/simple.mhdl".to_string()))
        .expect("No file path");
    let src = fs::read_to_string(path).expect("Failed to read file");

    match parser().parse(src.trim()).into_result() {
        Ok(ast) => ast.into_iter().for_each(|data| println!("{:?}", data)),
        Err(errs) => errs.into_iter().for_each(|e| println!("{:?}", e)),
    };
}
