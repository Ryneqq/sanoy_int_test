use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "reader", about = "")]
pub struct AppArgs {
    #[structopt(short = "f", long = "files")]
    pub files: usize,
    #[structopt(parse(from_os_str))]
    pub path: PathBuf,
    #[structopt(short = "s", long = "sort")]
    pub sort: bool,
}
