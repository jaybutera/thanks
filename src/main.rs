use serde::{Serialize, Deserialize};
use libipld::{
    Multihash,
    //codec::{Codec, Encode, Decode},
    DagCbor, Cid, cid
};
use std::fs::File;

mod types;
mod thunks;

#[derive(DagCbor, Serialize, Deserialize)]
pub struct Test {
    name: String,
    links: Vec<Cid>,
}

fn main() {
    //let res = thunks::get_thunk(&String::from("bafyreia2t6jpvrgccl3nnssskup3b3y4b6hjbybmggvivdy2yloab75ppm"));
    let res = thunks::get_thesis(&String::from("bafyreiaewuf45wcwmtetfsp7swh4kqj542pkmqde5swc6mom7ux6jn6gde"));
    //let h = Multihash::from_bytes("bafyreigv4gb3kcqzx2whnvzyivzi67mfifsj4cvziy3yecm5z5mfstzaqy".as_bytes()).unwrap();

    //use libipld::multihash::{Code, MultihashDigest, MultihashGeneric};
    //let multihash = Code::Sha3_256.digest(b"Hello world!");
    //let cid = cid::CidGeneric::new_v0(h).unwrap();
    let test = Test {
        name: "hello world".into(),
        links: vec![],
    };
    use libipld::json::DagJsonCodec;
    let codec = DagJsonCodec;

    let ferris_file = File::create("ferris.cbor").unwrap();
    // Write Ferris to the given file.
    // Instead of a file you can use any type that implements `io::Write`
    // like a HTTP body, database connection etc.
    serde_ipld_dagcbor::to_writer(ferris_file, &test).unwrap();

    //let ser = codec.encode(&test).unwrap();
    //println!("{ser}");
    //println!("{res:?}");
}
