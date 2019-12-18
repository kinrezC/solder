extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate structopt;

use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Contract {
    contractName: String,
    abi: Vec<AbiType>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AbiType {
    r#type: String,
    name: Option<String>,
    inputs: Vec<InputType>,
}

#[derive(Serialize, Deserialize, Debug)]
struct InputType {
    internalType: String,
    name: String,
    r#type: String,
}

pub fn read_contract_files(file_paths: &Vec<PathBuf>) -> Vec<Contract> {
    let mut contract_interfaces: Vec<Contract> = Vec::new();
    for file in file_paths {
        let content = read_to_string(&file).unwrap();
        let u: Contract = match serde_json::from_str(&content) {
            Err(why) => panic!("Error trying to deserialize: {}", why),
            Ok(deserialized) => deserialized,
        };
        contract_interfaces.push(u);
    }
    contract_interfaces
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
