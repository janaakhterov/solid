use serde::Deserialize;
use serde_json::{
    Map,
    Value,
};
use std::{
    fs,
    path::PathBuf,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "cargo-solid",
    about = "Solidity contract to Rust struct generator."
)]
struct Opt {
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

const OUTPUT: &str = "./src/solidity.rs";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SolidityAbi {
    contracts: Map<String, Value>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SolidityContract {
    abi: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SolidityField {
    constant: Option<bool>,
    inputs: Option<Vec<SolidityType>>,
    name: Option<String>,
    outputs: Option<Vec<SolidityType>>,
    payable: bool,
    stateMutability: String,
    r#type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SolidityType {
    internal_type: Option<String>,
    name: String,
    r#type: String,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    let contents = fs::read_to_string(&opt.input)?;

    let abi: SolidityAbi = serde_json::from_str(&contents)?;

    let contracts = abi
        .contracts
        .into_iter()
        .map(|(name, value)| {
            let value: Result<SolidityContract, _> = serde_json::from_value(value);
            let (filename, contract) = name.split_at(name.find(":").unwrap_or(0));
            let filename = filename.split_at(filename.find(".").unwrap_or(0)).0;
            let contract = contract.split_at(1).1;

            match value {
                Ok(value) => Ok((name, value)),
                Err(err) => Err(err),
            }
        })
        .collect::<Result<Vec<(String, SolidityContract)>, _>>()?
        .into_iter()
        .map(|(name, value)| {
            let value: Result<Vec<SolidityField>, _> = serde_json::from_str(&value.abi);
            match value {
                Ok(value) => Ok((name, value)),
                Err(err) => Err(err),
            }
        })
        .collect::<Result<Vec<(String, Vec<SolidityField>)>, _>>()?;

    Ok(())
}
