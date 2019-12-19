extern crate ethereum_types;
extern crate hex;
extern crate keccak_hash;
extern crate regex;
extern crate serde;
extern crate serde_any;
extern crate structopt;

use ethereum_types::H256;
use keccak_hash::keccak;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use serde_json;
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
    #[serde(default = "default_name")]
    name: String,
    inputs: Vec<InputType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct InputType {
    internalType: String,
    name: String,
    r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SelectorMatchContainer {
    name: String,
    selector_match: Vec<SelectorMatch>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SelectorMatch {
    signature: String,
    selector: String,
}

fn default_name() -> String {
    "unnamed".to_string()
}

pub fn process_functions_and_events(path: PathBuf) {
    let file_paths = get_valid_files_in_path(path);
    let abi_types = deserialize_json_interface(file_paths);
    let (function_types, event_types) = separate_functions_and_events(abi_types);
    let functions_name = "functions".to_string();
    let events_name = "events".to_string();
    let functions_match = parse_signatures_and_selectors(function_types, functions_name);
    let events_match = parse_signatures_and_selectors(event_types, events_name);

    let functions_json = serde_json::to_string(&functions_match).unwrap();
    let events_json = serde_json::to_string(&events_match).unwrap();

    serde_any::to_file("functions.json", &functions_json).unwrap();
    serde_any::to_file("events.json", &events_json).unwrap();
}

fn parse_signatures_and_selectors(vec: Vec<AbiType>, type_name: String) -> SelectorMatchContainer {
    let mut signatures_and_selectors: Vec<SelectorMatch> = Vec::new();
    for item in vec {
        let mut signature = (item.name + "(").to_string();
        if item.inputs.len() > 0 {
            for input in item.inputs {
                signature = (signature + &input.r#type.clone() + ", ").to_string();
            }
            signature.truncate(signature.len() - 2);
            signature = (signature + ")").to_string();
        } else {
            signature = (signature + ")").to_string();
        }
        let bytes = signature.as_bytes();
        let hash: H256 = keccak(bytes);
        let selector = "0x".to_string() + &hex::encode(&hash[0..4]);
        let selector_match = SelectorMatch {
            signature: signature,
            selector: selector,
        };

        signatures_and_selectors.push(selector_match);
    }

    SelectorMatchContainer {
        name: type_name,
        selector_match: signatures_and_selectors,
    }
}

fn separate_functions_and_events(contracts: Vec<Contract>) -> (Vec<AbiType>, Vec<AbiType>) {
    let mut function_types: Vec<AbiType> = Vec::new();
    let mut event_types: Vec<AbiType> = Vec::new();
    for iface in contracts {
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
