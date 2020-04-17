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
    Solid(Solid),
}

#[derive(Debug, StructOpt)]
struct Solid {
    // Support nightly
    #[structopt(long = "nightly")]
    nightly: bool,

    // Output directory
    #[structopt(short = "o", long = "output")]
    output: Option<PathBuf>,

    // Input files
    #[structopt(parse(from_os_str))]
    input: Vec<PathBuf>,
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

use anyhow::Result;
pub use solidity_contract::SolidityContract;
pub use solidity_field::SolidityField;
pub use solidity_type::SolidityType;

fn main() -> Result<()> {
    let opt = match Opt::from_args() {
        Opt::Solid(opt) => opt,
    };

    let (output, mut mod_file) = if let Some(output) = opt.output {
        // Ignore errors. Errors occur if directory already exists
        fs::create_dir(&output).ok();

        let mut mod_file = output.clone();
        mod_file.push(Path::new("mod.rs"));
        let mod_file = fs::File::create(&mod_file)?;

        (output, Some(mod_file))
    } else {
        (Path::new("src").to_path_buf(), None)
    };

    for file in opt.input {
        let contents = fs::read_to_string(&file)?;

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
                let mut output = output.clone();
                output.push(format!("{}.rs", filename));
                match fields {
                    Ok(fields) => Ok(SolidityContract {
                        filename: output,
                        contract,
                        fields,
                    }),
                    Err(err) => Err(err),
                }
            })
            .collect::<Result<Vec<SolidityContract>, _>>()?;

        // Create all the contract files. One file per contract.
        for contract in contracts.into_iter() {
            let filename = &contract.filename;
            let contract = format!(
                "{}{}",
                contract.types(opt.nightly),
                contract.functions(opt.nightly)
            );

            let mut file = fs::File::create(&filename)?;

            file.write(contract.as_bytes())?;
            if let (Some(mod_file), Some(Some(filename))) = (
                &mut mod_file,
                filename.file_name().map(|name| name.to_str()),
            ) {
                mod_file.write(
                    &format!("pub mod {};\n", filename.split(".").next().unwrap()).as_bytes(),
                )?;
            }
        }
    }

    Ok(())
}
