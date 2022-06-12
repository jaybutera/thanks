use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use std::path::Path;
use anyhow::Context;
use anyhow::Result;
use crate::grammar::types::{Reference, ThesisAst, ThunkAst, Ast};

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

    assert_eq!(pair.as_rule(), Rule::document);

    let children: Vec<_> = pair.into_inner().collect();

    let refs: Vec<Reference> = children.iter()
        .filter(|child| child.as_rule() == Rule::reference)
        .map(|child| parse_reference(child.clone()))
        .collect();

    let theses: Vec<ThesisAst> = children.iter()
        .filter(|child| child.as_rule() == Rule::reference)
        .map(|child| parse_thesis(child.clone()))
        .collect();

    Ok(Ast {
        refs,
        theses,
    })
}

fn parse_reference(pair: Pair<Rule>) -> Reference {
    let mut children = pair.into_inner();

    Reference {
        alias: children.next().unwrap().as_str().into(),
        hash: children.next().unwrap().as_str().into(),
    }
}

fn parse_thesis(pair: Pair<Rule>) -> ThesisAst {
    let mut children = pair.into_inner();

    let name = children.next().unwrap().as_str().into();
    let thunks = children.into_iter()
        .map(|child| parse_thunk(child.clone()))
        .collect();

    ThesisAst {
        name,
        thunks,
    }
}

fn parse_thunk(pair: Pair<Rule>) -> ThunkAst {
    let mut children = pair.into_inner();

    ThunkAst {
        alias: children.next().unwrap().as_str().into(),
        number: children.next().unwrap().as_str().parse().unwrap(),
        text: children.next().unwrap().as_str().into(),
    }
}
