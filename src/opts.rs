use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Opt {
    #[structopt(short, long)]
    pub import: Option<PathBuf>,

    #[structopt(short, long)]
    pub thesis: Option<String>,
}
