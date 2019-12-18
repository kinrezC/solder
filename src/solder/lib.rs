extern crate structopt;
extern crate regex;
extern crate serde_json;
extern crate serde;

use std::path::{PathBuf};
use std::fs::read_to_string;
use regex::Regex;
use serde_derive::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
struct Contract {
    name: String,
    abi: ABI
}

#[derive(Serialize, Deserialize, Debug)]
struct ABI {
    abi_contents: Vec<AbiType>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AbiType {
    object_type: String,
    anonymous: bool,
    name: String,
    constant: bool,
    state_mutability: String,
    payable: bool,
    inputs: Vec<InputType>,
    outputs: Vec<OutputType>
}

#[derive(Serialize, Deserialize, Debug)]
struct InputType {
    internal_type: String,
    name: String,
    input_type: String,
    indexed: bool
}

#[derive(Serialize, Deserialize, Debug)]
struct OutputType {
    internal_type: String,
    name: String,
    output_type: String
}

pub fn read_contract_files(file_paths: &Vec<PathBuf>) {
    for file in file_paths {
        let content = read_to_string(&file).unwrap();
        let u: Contract = serde_json::from_str(&content).unwrap();
        print!("{:?}", u);
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
