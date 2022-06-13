use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use std::path::Path;
use anyhow::Context;
use crate::grammar::{
    context::{Ctx, ToCtx, CtxLocation, CtxResult, ToCtxErr, FileId},
    types::{Reference, ThesisAst, ThunkAst, Ast},
};

#[derive(Parser)]
#[grammar = "grammar/grammar.pest"]
struct RawParser;

pub fn parse_doc(input: &str, root: &Path) -> CtxResult<Ctx<Ast>> {
    let source = internment::Intern::new(root.to_str().unwrap().into());
    let root_ctx = CtxLocation {
        source: internment::Intern::new(root.to_str().unwrap().into()),
        start_offset: 0,
        end_offset: 0,
    };

    let pair = RawParser::parse(Rule::document, input)
        .map_err(|err| {
            let location = err.location.clone();
            let (start_offset, end_offset) = match location {
                pest::error::InputLocation::Pos(p) => (p, p),
                pest::error::InputLocation::Span(p) => p,
            };
            println!("{err:?}");
            let message = err
                .to_string()
                .lines()
                .last()
                .unwrap()
                .trim()
                .trim_start_matches('=')
                .trim()
                .to_string();

            anyhow::anyhow!("{}", message).with_ctx(CtxLocation {
                source,
                start_offset,
                end_offset,
            })
        })?
        .next()
        .context("no pairs produced in parser")?;
        //.err_ctx(Some(root_ctx))?;

    assert_eq!(pair.as_rule(), Rule::document);

    let ctx = p2ctx(&pair, source);
    let children: Vec<_> = pair.into_inner().collect();

    let refs: Vec<Ctx<Reference>> = children.iter()
        .filter(|child| child.as_rule() == Rule::reference)
        .map(|child| parse_reference(child.clone(), source))
        .collect();

    let theses: Vec<Ctx<ThesisAst>> = children.iter()
        .filter(|child| child.as_rule() == Rule::reference)
        .map(|child| parse_thesis(child.clone(), source))
        .collect();

    Ok(Ast {
        refs,
        theses,
    }.with_ctx(ctx))
}

fn parse_reference(pair: Pair<Rule>, source: FileId) -> Ctx<Reference> {
    let ctx = p2ctx(&pair, source);
    let mut children = pair.into_inner();

    Reference {
        alias: children.next().unwrap().as_str().into(),
        hash: children.next().unwrap().as_str().into(),
    }.with_ctx(ctx)
}

fn parse_thesis(pair: Pair<Rule>, source: FileId) -> Ctx<ThesisAst> {
    let ctx = p2ctx(&pair, source);
    let mut children = pair.into_inner();

    let name = children.next().unwrap().as_str().into();
    let thunks = children.into_iter()
        .map(|child| parse_thunk(child.clone(), source))
        .collect();

    ThesisAst {
        name,
        thunks,
    }.with_ctx(ctx)
}

fn parse_thunk(pair: Pair<Rule>, source: FileId) -> Ctx<ThunkAst> {
    let ctx = p2ctx(&pair, source);
    let mut children = pair.into_inner();

    ThunkAst {
        alias: children.next().unwrap().as_str().into(),
        number: children.next().unwrap().as_str().parse().unwrap(),
        text: children.next().unwrap().as_str().into(),
    }.with_ctx(ctx)
}

fn p2ctx(pair: &Pair<Rule>, source: FileId) -> CtxLocation {
    CtxLocation {
        source,
        start_offset: pair.as_span().start(),
        end_offset: pair.as_span().end(),
    }
}
