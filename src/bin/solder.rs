extern crate structopt;
extern crate solder;

use std::path::{PathBuf, Path};
use structopt::StructOpt;
use std::fs::read_to_string;

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
    
    solder::read_file(&files);
}



