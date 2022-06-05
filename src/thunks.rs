use crate::types::{Hash, Thunk};
use std::{process::{Command, Stdio}, io::Read};
use anyhow::Result;

pub fn get_thunk(hash: &Hash) -> Result<Thunk> {
    let child = Command::new("ipfs").arg("dag").arg("get").arg(hash)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run ipfs get dag");

    let mut buf: String = String::from("");
    child.stdout.unwrap().read_to_string(&mut buf)?;
    println!("{buf}");
    let res: Thunk = serde_json::from_str(&buf)?;
    Ok(res)
}
