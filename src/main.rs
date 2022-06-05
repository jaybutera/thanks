//use ipfs::dag::IpldDag;
mod types;
mod thunks;

fn main() {
    let res = thunks::get_thunk(&String::from("bafyreia2t6jpvrgccl3nnssskup3b3y4b6hjbybmggvivdy2yloab75ppm"));
    println!("{res:?}");
}
