use crate::types::{Hash, Thunk, Thesis, DagJsonThunk, DagJsonThesis};
use std::{process::{Command, Stdio}, io::Read};
use anyhow::Result;

pub fn save_thunk(thunk: &Thunk) -> Result<Hash> {
    let ser = serde_json::to_string(thunk)?.replace("\"", "\\\"");
    let echo = Command::new("echo").arg(format!("\"{ser}\""))
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run echo");
    let child = Command::new("ipfs").arg("dag").arg("put").arg("--pin=true")//.arg(format!("\"{ser}\""))
        .stdin(echo.stdout.unwrap())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run ipfs get dag");

    let mut buf: String = String::from("");
    child.stdout.unwrap().read_to_string(&mut buf)?;

    Ok(buf)
}

pub fn get_thesis(hash: &Hash) -> Result<Thesis> {
    let child = Command::new("ipfs").arg("dag").arg("get").arg(hash)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run ipfs get dag");

    let mut buf: String = String::from("");
    child.stdout.unwrap().read_to_string(&mut buf)?;
    println!("{buf}");
    let res: DagJsonThesis = serde_json::from_str(&buf)?;
    Ok(res.into())
}

/*
pub fn get_thunk(hash: &Hash) -> Result<Thunk> {
    let child = Command::new("ipfs").arg("dag").arg("get").arg(hash)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run ipfs get dag");

    //let mut buf: String = String::from("");
    //let mut buf: Vec<u8> = String::from("");
    //child.stdout.unwrap().read_buf(&mut buf)?;
    //child.stdout.unwrap().read()?;
    //let res: DagJsonThunk = serde_json::from_str(&buf)?;
    Ok(res.into())
}
*/
