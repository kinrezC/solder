extern crate structopt;
extern crate regex;
extern crate serde;

use std::path::{Path, PathBuf};
use structopt::StructOpt;
use std::fs::read_to_string;
use serde::Deserialize;
use regex::Regex;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

#[derive(Deserialize, Debug)]
struct ABI {
        
}

#[derive(Deserialize, Debug)]
struct Contract {
    name: String,
    abi: ABI,
}





pub fn read_contract_files(file_paths: &Vec<PathBuf>) {
    for file in file_paths {
        let content = read_to_string(&file).unwrap();
        print!("{:?}", content);
    }
}

pub fn get_valid_files_in_path(path: &PathBuf) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    let json = Regex::new("json$").unwrap();
    for entry in path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            if entry.path().is_file() {
                if json.is_match(entry.path().to_str().unwrap()) {
                    files.push(entry.path());
                }
            }
        }
    }
    files

}
