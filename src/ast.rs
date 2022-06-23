use anyhow::anyhow;
use crate::types::{Thunk, Hash};
use crate::grammar::{
    types::RawAst,
    context::{ToCtx, CtxResult},
};

pub fn thunks(raw: RawAst) -> CtxResult<Vec<Thunk>> {
    let mut thunks = vec![];

    for thesis in raw.theses {
        for thunk in thesis.thunks.iter() {
            let refs = thunk.refs.iter()
                .map(|(_, alias)| raw.refs.get(alias)
                    .map(|h| h.clone())
                    .ok_or_else(|| anyhow!(format!("Couldn't find referenced thesis {alias} in reference list")).with_ctx(thunk.ctx())))
                .collect::<CtxResult<Vec<Hash>>>()?;

            thunks.push(Thunk {
                refs: refs.clone(),
                text: thunk.text.clone(),
            })
        }

    }

    Ok(thunks)
}
