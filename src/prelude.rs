pub use std::{env, fmt, fs};

pub use ariadne::{Color, Label, Report, ReportKind, sources};

pub use chumsky::input::*;
pub use chumsky::prelude::*;

pub use crate::lexer::*;
pub use crate::parser::*;

pub type Span = SimpleSpan;
pub type Spanned<T> = (T, Span);
