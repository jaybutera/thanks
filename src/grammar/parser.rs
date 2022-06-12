use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use std::path::Path;
use crate::types::Ast;
use anyhow::Context;
use anyhow::Result;

#[derive(Parser)]
#[grammar = "grammar/grammar.pest"]
struct RawParser;

pub fn parse_doc(input: &str, root: &Path) -> Result<Ast> {
    let pair = RawParser::parse(Rule::document, input)
        .map_err(|err| {
            let location = err.location.clone();
            let (start_offset, end_offset) = match location {
                pest::error::InputLocation::Pos(p) => (p, p),
                pest::error::InputLocation::Span(p) => p,
            };
            let message = err
                .to_string()
                .lines()
                .last()
                .unwrap()
                .trim()
                .trim_start_matches('=')
                .trim()
                .to_string();

            anyhow::anyhow!("{}", message)
        })?
        .next()
        .context("no pairs produced in parser")?;

    Ok(Ast {
        refs: vec![],
        theses: vec![],
    })
}
