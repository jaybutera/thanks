use serde::{Serialize, Deserialize};
use std::fs::File;
use types::{Index, Hash, Thunk, Thesis};
use structopt::StructOpt;
use std::path::PathBuf;
use anyhow::Result;
use index::{get_index, save_index};
use grammar::parser;

mod types;
mod thunks;
mod opts;
mod index;
mod grammar;

fn parse_notes(file: PathBuf) -> Result<Vec<String>> {
    let content = std::fs::read_to_string(file)?;
    Ok( content.split("\n\n").map(|s| s.to_string()).collect::<Vec<_>>() )
}

fn main() {
    let opt = opts::Opt::from_args();

    if let Some(filepath) = opt.import {
        let content = std::fs::read_to_string(filepath.clone()).expect("Couldn't open file");
        let res = parser::parse_doc(&content, &filepath);
        println!("{res:?}");
        /*
        match parse_notes(filepath.clone()) {
            Ok(notes) => {
                let hashes: Vec<Hash> = notes.into_iter()
                    .map(|n| thunks::save_thunk(n.into()).unwrap())
                    .collect();

                let name = filepath.file_stem().unwrap().to_string_lossy();
                let thesis_hash = thunks::save_thesis(Thesis {
                    name: name.clone().into(),
                    refs: hashes,
                }).expect("Failed to save thesis");

                // Update thesis pointer in index
                let mut index = get_index()
                    .expect("Failed to get index");
                index.theses.insert(name.into(), thesis_hash.clone());
                save_index(index)
                    .expect("Failed to save index");

                println!("{thesis_hash}");
            }
            Err(e) => println!("{e}"),
        }
        */
    } else if let Some(thesis_name) = opt.thesis {
        // TODO wrap up the unwraps
        let index = get_index().unwrap();
        let hash  = index.theses.get(&thesis_name).unwrap();
        let thesis = thunks::get_thesis(hash).unwrap();
        println!("{thesis}");
    }

    /*
    let thunk = Thunk {
        text: "Terracotta soldier".into(),
        refs: vec![],
    };

    let multihash = thunks::save_thunk(thunk).unwrap();
    println!("{multihash}");
    */
    //let res = thunks::get_thunk(&String::from("bafyreia2t6jpvrgccl3nnssskup3b3y4b6hjbybmggvivdy2yloab75ppm"));
    //let res = thunks::get_thesis(&String::from("bafyreiaewuf45wcwmtetfsp7swh4kqj542pkmqde5swc6mom7ux6jn6gde"));

    //let ser = codec.encode(&test).unwrap();
    //println!("{ser}");
    //println!("{res:?}");
}
