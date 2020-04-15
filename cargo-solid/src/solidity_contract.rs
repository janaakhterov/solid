use super::SolidityField;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SolidityContract {
    pub filename: String,
    pub contract: String,
    pub fields: Vec<SolidityField>,
}

impl SolidityContract {
    pub fn functions(&self) -> String {
        let functions = self
            .fields
            .iter()
            .map(|field| field.to_rust_function())
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

    pub fn types(&self) -> String {
        format!(
            "\
#[allow(unused_imports)]
use solid::{{derive::Encode, Encode}};\n\n{}",
            self.fields
                .iter()
                .map(|field| field.get_output_type())
                .filter(Option::is_some)
                .map(Option::unwrap)
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
