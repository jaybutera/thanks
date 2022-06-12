use crate::types::Hash;

#[derive(Debug)]
pub struct Reference {
    pub alias: String,
    pub hash: Hash,
}

#[derive(Debug)]
pub struct ThunkAst {
    pub alias: String,
    pub number: u32,
    pub text: String,
}

#[derive(Debug)]
pub struct ThesisAst {
    pub name: String,
    pub thunks: Vec<ThunkAst>,
}

#[derive(Debug)]
pub struct Ast {
    pub refs: Vec<Reference>,
    pub theses: Vec<ThesisAst>,
}
