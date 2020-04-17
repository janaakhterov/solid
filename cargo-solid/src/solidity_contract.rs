use super::SolidityField;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct SolidityContract {
    pub filename: PathBuf,
    pub contract: String,
    pub fields: Vec<SolidityField>,
}

impl SolidityContract {
    pub fn functions(&self, nightly: bool) -> String {
        let functions = self
            .fields
            .iter()
            .map(|field| field.to_rust_function(nightly))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "
pub struct {contract};

impl {contract} {{
{functions}
}}
",
            contract = self.contract,
            functions = functions
        )
    }

    pub fn types(&self, nightly: bool) -> String {
        format!(
            "\
#[allow(unused_imports)]
use solid::{{derive::Encode, Encode}};\n\n{}",
            self.fields
                .iter()
                .map(|field| field.get_output_type(nightly))
                .filter(Option::is_some)
                .map(Option::unwrap)
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
