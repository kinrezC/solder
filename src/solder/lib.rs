extern crate regex;
extern crate serde;
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AbiType {
    r#type: String,
    name: Option<String>,
    inputs: Vec<InputType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct InputType {
    internalType: String,
    name: String,
    r#type: String,
}

pub fn process_signatures_and_selectors(path: PathBuf) -> (Vec<AbiType>, Vec<AbiType>) {
    let file_paths = get_valid_files_in_path(path);
    let contract_interfaces = deserialize_json_interface(file_paths);

    let mut function_types: Vec<AbiType> = Vec::new();
    let mut event_types: Vec<AbiType> = Vec::new();
    for iface in contract_interfaces {
        for abi_type in iface.abi {
            match abi_type.r#type.as_ref() {
                "function" => function_types.push(abi_type.clone()),
                "event" => event_types.push(abi_type.clone()),
                _ => (),
            }
        }
    }
    (function_types, event_types)
}

fn deserialize_json_interface(file_paths: Vec<PathBuf>) -> Vec<Contract> {
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

fn get_valid_files_in_path(path: PathBuf) -> Vec<PathBuf> {
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
