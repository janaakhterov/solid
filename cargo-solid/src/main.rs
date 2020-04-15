use serde::Deserialize;
use serde_json::{
    Map,
    Value,
};
use std::{
    fs,
    io::Write,
    path::{
        Path,
        PathBuf,
    },
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    bin_name = "cargo",
    about = "Solidity contract to Rust struct generator."
)]
enum Opt {
    #[structopt(name = "solid")]
    Solid {
        #[structopt(parse(from_os_str))]
        input: PathBuf,
    },
}

#[derive(Debug, Deserialize)]
struct SolidityAbi {
    contracts: Map<String, Value>,
}

#[derive(Debug, Deserialize)]
struct SolidityAbiContract {
    abi: String,
}

pub mod solidity_contract;
pub mod solidity_field;
pub mod solidity_type;

pub use solidity_contract::SolidityContract;
pub use solidity_field::SolidityField;
pub use solidity_type::SolidityType;

fn main() -> anyhow::Result<()> {
    let input = match Opt::from_args() {
        Opt::Solid { input } => input,
    };

    let contents = fs::read_to_string(&input)?;

    let abi: SolidityAbi = serde_json::from_str(&contents)?;

    let contracts = abi
        .contracts
        .into_iter()
        .map(|(name, value)| {
            let value: Result<SolidityAbiContract, _> = serde_json::from_value(value);
            let (filename, contract) = name.split_at(name.find(":").unwrap_or(0));
            let filename = filename
                .split_at(filename.find(".").unwrap_or(0))
                .0
                .to_string();
            let contract = contract.split_at(1).1.to_string();

            match value {
                Ok(value) => Ok((filename, contract, value)),
                Err(err) => Err(err),
            }
        })
        .collect::<Result<Vec<(String, String, SolidityAbiContract)>, _>>()?
        .into_iter()
        .map(|(filename, contract, value)| {
            let fields: Result<Vec<SolidityField>, _> = serde_json::from_str(&value.abi);
            match fields {
                Ok(fields) => Ok(SolidityContract {
                    filename: format!("./src/{}.rs", filename),
                    contract,
                    fields,
                }),
                Err(err) => Err(err),
            }
        })
        .collect::<Result<Vec<SolidityContract>, _>>()?;

    // println!("{:#?}", contracts);

    // Create all the contract files. One file per contract.
    for contract in contracts.into_iter() {
        let filename = &contract.filename;
        let contract = format!("{}{}", contract.types(), contract.functions());

        let mut file = fs::File::create(Path::new(filename))?;

        file.write(contract.as_bytes())?;
    }

    Ok(())
}
