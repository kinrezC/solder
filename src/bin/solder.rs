extern crate solder;
extern crate structopt;

use std::fs::read_to_string;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let path = args.path;

    let files: Vec<PathBuf> = solder::get_valid_files_in_path(&path);
    let contract_interfaces = solder::read_contract_files(&files);
    println!("{:?}", contract_interfaces);
}
