use super::SolidityType;
use heck::SnakeCase;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolidityField {
    pub constant: Option<bool>,
    pub inputs: Option<Vec<SolidityType>>,
    pub name: Option<String>,
    pub outputs: Option<Vec<SolidityType>>,
    pub payable: Option<bool>,
    pub state_mutability: String,
    pub r#type: String,
}

impl SolidityField {
    pub fn to_rust_function(&self) -> String {
        let inputs = if let Some(inputs) = &self.inputs {
            inputs
                .iter()
                .map(|ty| ty.to_rust_declaration(false))
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            String::new()
        };

        let variables = if let Some(inputs) = &self.inputs {
            inputs
                .iter()
                .map(|input| format!("\n\t\t\t.push({})", input.name))
                .collect::<Vec<_>>()
                .join("")
        } else {
            String::new()
        };

        let function = if let Some(ref name) = self.name {
            name.to_snake_case()
        } else {
            "new".to_string()
        };

        let name = if let Some(ref name) = self.name {
            format!("\n\t\t\t.name(\"{}\")", name.as_str())
        } else {
            "".to_string()
        };

        match self.r#type.as_str() {
            "constructor" | "function" => format!(
                "
    #[rustfmt::skip]
    #[allow(dead_code)]
    pub fn {}({}) -> Vec<u8> {{
        solid::Builder::new(){}{}
            .build()
    }}",
                function, inputs, variables, name
            ),

            _ => String::new(),
        }
    }

    pub fn get_output_type(&self) -> Option<String> {
        if let (Some(outputs), Some(name)) = (&self.outputs, &self.name) {
            let named = outputs
                .iter()
                .filter(|r#type| r#type.name.len() > 0)
                .count();

            if named != outputs.len() || outputs.len() == 0 {
                return None;
            }

            let name = match name.chars().next() {
                None => return None,
                Some(first) => {
                    first.to_uppercase().collect::<String>() + name.split_at(1).1 + "Output"
                }
            };

            let lifetime = if outputs
                .iter()
                .filter(|r#type| r#type.has_lifetime())
                .count()
                == 0
            {
                String::new()
            } else {
                "<'a>".to_string()
            };

            let declarations = outputs
                .iter()
                .map(|r#type| format!("\tpub {},", r#type.to_rust_declaration(true)))
                .collect::<Vec<_>>()
                .join("\n");

            match self.r#type.as_str() {
                "function" => Some(format!(
                    "\
#[derive(Encode)]
pub struct {}{} {{
{}
}}
                        ",
                    name, lifetime, declarations
                )),
                _ => None,
            }
        } else {
            None
        }
    }
}
