//use ipfs::dag::IpldDag;
mod types;
mod thunks;

fn main() {
    //let res = thunks::get_thunk(&String::from("bafyreia2t6jpvrgccl3nnssskup3b3y4b6hjbybmggvivdy2yloab75ppm"));
    let res = thunks::get_thesis(&String::from("bafyreiaewuf45wcwmtetfsp7swh4kqj542pkmqde5swc6mom7ux6jn6gde"));
    println!("{res:?}");
}
