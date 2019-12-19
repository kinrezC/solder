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
    let pattern = args.pattern;
    let path = args.path;

    match pattern.as_ref() {
        "test" => solder::process_functions_and_events(path),
        _ => println!("Undefined command: {}", pattern),
    }
}
