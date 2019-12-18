extern crate solder;
extern crate structopt;

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

    let (function_types, event_types) = solder::process_signatures_and_selectors(path);

    println!("{:?} and {:?}", function_types, event_types);
}
