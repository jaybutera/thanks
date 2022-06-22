use crate::types::Hash;
use crate::grammar::context::Ctx;
use internment::Intern;

#[derive(Debug)]
pub struct Reference {
    pub alias: String,
    pub hash: Hash,
}

#[derive(Debug)]
pub struct ThunkAst {
    pub refs: Ctx<Vec<(u64, Intern<String>)>>,
    pub text: String,
}

#[derive(Debug)]
pub struct ThesisAst {
    pub name: String,
    pub thunks: Vec<Ctx<ThunkAst>>,
}

#[derive(Debug)]
pub struct Ast {
    pub refs: Vec<Ctx<Reference>>,
    pub theses: Vec<Ctx<ThesisAst>>,
}
