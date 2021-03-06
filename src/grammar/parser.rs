use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use std::path::Path;
use anyhow::Context;
use internment::Intern;
use crate::types::Hash;
use crate::grammar::{
    context::{Ctx, ToCtx, CtxLocation, CtxResult, FileId},
    types::{RefMap, ThesisAst, ThunkAst, RawAst},
};

#[derive(Parser)]
#[grammar = "grammar/grammar.pest"]
struct RawParser;

pub fn parse_doc(input: &str, root: &Path) -> CtxResult<Ctx<RawAst>> {
    let source = Intern::new(root.to_str().unwrap().into());
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

    println!("{pair:?}");
    assert_eq!(pair.as_rule(), Rule::document);

    let ctx = p2ctx(&pair, source);
    let children: Vec<_> = pair.into_inner().collect();

    let refs: RefMap = children.iter()
        .filter(|child| child.as_rule() == Rule::reference)
        // TODO don't just discard the context
        .map(|child| (*parse_reference(child.clone(), source)).clone())
        .collect();

    let theses: Vec<Ctx<ThesisAst>> = children.iter()
        .filter(|child| child.as_rule() == Rule::thesis)
        .map(|child| parse_thesis(child.clone(), source))
        .collect();

    Ok(RawAst {
        refs,
        theses,
    }.with_ctx(ctx))
}

fn parse_reference(pair: Pair<Rule>, source: FileId) -> Ctx<(Intern<String>, Hash)> {
    let ctx = p2ctx(&pair, source);
    let mut children = pair.into_inner();

    let alias: Intern<String> = Intern::new(children.next().unwrap().as_str().into());
    let hash: Hash  = children.next().unwrap().as_str().into();

    (alias, hash).with_ctx(ctx)
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

    let refs = parse_thunk_refs(children.next().unwrap(), source);

    ThunkAst {
        refs,
        text: children.next().unwrap().as_str().into(),
    }.with_ctx(ctx)
}

fn parse_thunk_refs(pair: Pair<Rule>, source: FileId) -> Ctx<Vec<(u64, Intern<String>)>> {
    let ctx = p2ctx(&pair, source);
    let children = pair.into_inner().collect::<Vec<_>>();

    let mut refs = vec![];
    for pair in children.chunks(2) {
        let alias: Intern<String> = Intern::new(pair[1].as_str().into());
        let number: u64 = pair[0].as_str().parse().unwrap();
        refs.push((number, alias));
    }

    refs.with_ctx(ctx)
}

fn p2ctx(pair: &Pair<Rule>, source: FileId) -> CtxLocation {
    CtxLocation {
        source,
        start_offset: pair.as_span().start(),
        end_offset: pair.as_span().end(),
    }
}
