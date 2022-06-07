use serde::{Serialize, Deserialize};
use std::fs::File;
use types::Thunk;
use structopt::StructOpt;
use std::path::PathBuf;
use anyhow::Result;

mod types;
mod thunks;
mod opts;

fn parse_notes(file: PathBuf) -> Result<Vec<String>> {
    let content = std::fs::read_to_string(file)?;
    Ok( content.split("\n\n").map(|s| s.to_string()).collect::<Vec<_>>() )
}

fn main() {
    let opt = opts::Opt::from_args();

    if let Some(filepath) = opt.import {
        let notes = parse_notes(filepath);
        println!("{notes:?}");
    }

    let thunk = Thunk {
        text: "Terracotta soldier".into(),
        refs: vec![],
    };

    let multihash = thunks::save_thunk(&thunk).unwrap();
    println!("{multihash}");
    //let res = thunks::get_thunk(&String::from("bafyreia2t6jpvrgccl3nnssskup3b3y4b6hjbybmggvivdy2yloab75ppm"));
    //let res = thunks::get_thesis(&String::from("bafyreiaewuf45wcwmtetfsp7swh4kqj542pkmqde5swc6mom7ux6jn6gde"));

    //let ser = codec.encode(&test).unwrap();
    //println!("{ser}");
    //println!("{res:?}");
}
