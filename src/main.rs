use crate::prelude::*;

mod lexer;
// mod parser;
mod prelude;

fn failure(
    msg: String,
    label: (String, SimpleSpan),
    extra_labels: impl IntoIterator<Item = (String, SimpleSpan)>,
    src: &str,
) -> ! {
    let fname = "example";
    Report::build(ReportKind::Error, (fname, label.1.into_range()))
        .with_message(&msg)
        .with_label(
            Label::new((fname, label.1.into_range()))
                .with_message(label.0)
                .with_color(Color::Red),
        )
        .with_labels(extra_labels.into_iter().map(|label2| {
            Label::new((fname, label2.1.into_range()))
                .with_message(label2.0)
                .with_color(Color::Yellow)
        }))
        .finish()
        .print(sources([(fname, src)]))
        .unwrap();
    std::process::exit(1)
}

fn parse_failure(err: &Rich<impl fmt::Display>, src: &str) -> ! {
    failure(
        err.reason().to_string(),
        (
            err.found()
                .map(|c| c.to_string())
                .unwrap_or_else(|| "end of input".to_string()),
            *err.span(),
        ),
        err.contexts()
            .map(|(l, s)| (format!("while parsing this {l}"), *s)),
        src,
    )
}

fn main() {
    let path = env::args()
        .nth(1)
        .or_else(|| Some("./test/simple.mhdl".to_string()))
        .expect("No file path");
    let src = fs::read_to_string(path).expect("Failed to read file");

    let tokens = lexer()
        .parse(src.trim())
        .into_result()
        .unwrap_or_else(|errs| parse_failure(&errs[0], src.trim()));

    tokens.into_iter().for_each(|data| println!("{:?}", data));
}
