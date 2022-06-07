use serde::{Serialize, Deserialize};
use std::fs::File;
use types::Thunk;

mod types;
mod thunks;

fn main() {
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
