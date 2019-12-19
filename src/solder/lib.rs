extern crate hex;
extern crate serde;
extern crate serde_json;

use ethereum_types::H256;
use keccak_hash::keccak;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use std::fs::{read_to_string, File};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Contract {
    contractName: String,
    abi: Vec<AbiType>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractWithData {
    contract_name: String,
    functions: Vec<AbiType>,
    events: Vec<AbiType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AbiType {
    r#type: String,
    #[serde(default = "default_name")]
    name: String,
    inputs: Vec<InputType>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct InputType {
    name: String,
    r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContractMatchContainer {
    contract_matches: Vec<ContractMatch>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ContractMatch {
    name: String,
    functions: Vec<SelectorMatch>,
    events: Vec<SelectorMatch>,
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
    let contracts_with_data = separate_functions_and_events(abi_types);
    let contract_match_container = process_signatures_and_selectors(contracts_with_data);
    let contract_matches_file = File::create("contract-selectors.json").unwrap();
    serde_json::to_writer_pretty(contract_matches_file, &contract_match_container).unwrap();
}

fn process_signatures_and_selectors(
    contracts_with_data: Vec<ContractWithData>,
) -> ContractMatchContainer {
    let mut contract_matches: Vec<ContractMatch> = Vec::new();

    for contract in contracts_with_data {
        let function_selector_matches = parse_signatures_and_selectors(contract.functions);
        let event_selector_matches = parse_signatures_and_selectors(contract.events);

        let contract_match = ContractMatch {
            name: contract.contract_name,
            functions: function_selector_matches,
            events: event_selector_matches,
        };
        contract_matches.push(contract_match)
    }
    let contract_match_container = ContractMatchContainer {
        contract_matches: contract_matches,
    };
    contract_match_container
}

fn parse_signatures_and_selectors(vec: Vec<AbiType>) -> Vec<SelectorMatch> {
    let mut selector_matches: Vec<SelectorMatch> = Vec::new();
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
        selector_matches.push(selector_match);
    }
    selector_matches
}

fn separate_functions_and_events(contracts: Vec<Contract>) -> Vec<ContractWithData> {
    let mut contracts_with_data: Vec<ContractWithData> = Vec::new();
    for iface in contracts {
        let mut function_types: Vec<AbiType> = Vec::new();
        let mut event_types: Vec<AbiType> = Vec::new();
        for abi_type in iface.abi {
            match abi_type.r#type.as_ref() {
                "function" => function_types.push(abi_type.clone()),
                "event" => event_types.push(abi_type.clone()),
                _ => (),
            }
        }
        let contract_with_data = ContractWithData {
            contract_name: iface.contractName,
            functions: function_types,
            events: event_types,
        };
        contracts_with_data.push(contract_with_data);
    }
    contracts_with_data
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
