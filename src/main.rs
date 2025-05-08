use crate::prelude::*; 

mod lexer;
mod parser;
mod prelude;

fn main() {
    let filename = env::args()
        .nth(1)
        .or_else(|| Some("./test/simple.mhdl".to_string()))
        .expect("No file path");
    let src = fs::read_to_string(&filename).expect("Failed to read file");

    let (tokens, errs) = lexer().parse(src.as_str()).into_output_errors();

    if let Some(tokens) = &tokens {
        tokens.iter().for_each(|data| println!("{:?}", data));
    }

    println!();

    let parse_errs = if let Some(tokens) = &tokens {
        let (ast, parse_errs) = scopes_parser()
            .parse(
                tokens
                    .as_slice()
                    .map((src.len()..src.len()).into(), |(t, s)| (t, s)),
            )
            .into_output_errors();

        if let Some(ast) = &ast {
            ast.iter().for_each(|data| println!("{:?}", data));
        }

        parse_errs
    } else {
        Vec::new()
    };

    errs.into_iter()
        .map(|e| e.map_token(|c| c.to_string()))
        .chain(
            parse_errs
                .into_iter()
                .map(|e| e.map_token(|tok| tok.to_string())),
        )
        .for_each(|e| {
            Report::build(ReportKind::Error, (filename.clone(), e.span().into_range()))
                .with_message(e.to_string())
                .with_label(
                    Label::new((filename.clone(), e.span().into_range()))
                        .with_message(e.reason().to_string())
                        .with_color(Color::Red),
                )
                .with_labels(e.contexts().map(|(label, span)| {
                    Label::new((filename.clone(), span.into_range()))
                        .with_message(format!("while parsing this {}", label))
                        .with_color(Color::Yellow)
                }))
                .finish()
                .print(sources([(filename.clone(), src.clone())]))
                .unwrap()
        });
}
