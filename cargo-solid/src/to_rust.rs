use super::{
    SolidityContract,
    SolidityField,
    SolidityType,
};

pub trait ToRust {
    fn to_rust(&self) -> String;
}

impl<T> ToRust for Vec<T>
where
    T: ToRust,
{
    fn to_rust(&self) -> String {
        self.iter()
            .map(ToRust::to_rust)
            .collect::<Vec<_>>()
            .join(",")
    }
}

impl ToRust for SolidityType {
    fn to_rust(&self) -> String {
        let ty = type_to_rust(self.r#type.as_str());
        format!("{}: {}", self.name, ty)
    }
}

impl ToRust for SolidityField {
    fn to_rust(&self) -> String {
        let inputs = if let Some(inputs) = &self.inputs {
            inputs
                .iter()
                .map(ToRust::to_rust)
                .collect::<Vec<_>>()
                .join(",")
        } else {
            "".to_string()
        };

        let variables = if let Some(inputs) = &self.inputs {
            inputs
                .iter()
                .map(|input| format!(".push({})", input.name))
                .collect::<Vec<_>>()
                .join("\n")
        } else {
            "".to_string()
        };

        match self.r#type.as_str() {
            "constructor" | "function" => format!(
                "
    pub fn {function}({inputs}) -> Vec<u8> {{
        solid::Builder::new()
            {variables}
            {name}
            .build()
    }}",
                function = if let Some(ref name) = self.name {
                    name.as_str()
                } else {
                    "new"
                },
                inputs = inputs,
                variables = variables,
                name = if let Some(ref name) = self.name {
                    format!(".name(\"{}\")", name.as_str())
                } else {
                    "".to_string()
                }
            ),
            _ => String::new(),
        }
    }
}

impl ToRust for SolidityContract {
    fn to_rust(&self) -> String {
        let functions = self
            .fields
            .iter()
            .map(ToRust::to_rust)
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
}

fn type_to_rust(ty: &str) -> String {
    if ty.ends_with("[]") {
        format!("Vec<{}>", type_to_rust(ty.split_at(ty.len() - 2).0))
    } else {
        match ty {
            "bytes" => "soild::Bytes<'_>",
            "string" => "&str",

            "int8" => "i8",
            "uint8" => "u8",
            "int16" => "i16",
            "uint16" => "u16",
            "int32" => "i32",
            "uint32" => "u32",
            "int64" => "i64",
            "uint64" => "u64",
            "int128" => "i128",
            "uint128" => "u128",

            "int24" => "solid::Int24",
            "uint24" => "solid::Uint24",
            "int40" => "solid::Int40",
            "uint40" => "solid::Uint40",
            "int48" => "solid::Int48",
            "uint48" => "solid::Uint48",
            "int56" => "solid::Int56",
            "uint56" => "solid::Uint56",
            "int72" => "solid::Int72",
            "uint72" => "solid::Uint72",
            "int80" => "solid::Int80",
            "uint80" => "solid::Uint80",
            "int88" => "solid::Int88",
            "uint88" => "solid::Uint88",
            "int96" => "solid::Int96",
            "uint96" => "solid::Uint96",
            "int104" => "solid::Int104",
            "uint104" => "solid::Uint104",
            "int112" => "solid::Int112",
            "uint112" => "solid::Uint112",
            "int120" => "solid::Int120",
            "uint120" => "solid::Uint120",
            "int136" => "solid::Int136",
            "uint136" => "solid::Uint136",
            "int144" => "solid::Int144",
            "uint144" => "solid::Uint144",
            "int152" => "solid::Int152",
            "uint152" => "solid::Uint152",
            "int160" => "solid::Int160",
            "uint160" => "solid::Uint160",
            "int168" => "solid::Int168",
            "uint168" => "solid::Uint168",
            "int176" => "solid::Int176",
            "uint176" => "solid::Uint176",
            "int184" => "solid::Int184",
            "uint184" => "solid::Uint184",
            "int192" => "solid::Int192",
            "uint192" => "solid::Uint192",
            "int200" => "solid::Int200",
            "uint200" => "solid::Uint200",
            "int208" => "solid::Int208",
            "uint208" => "solid::Uint208",
            "int216" => "solid::Int216",
            "uint216" => "solid::Uint216",
            "int224" => "solid::Int224",
            "uint224" => "solid::Uint224",
            "int232" => "solid::Int232",
            "uint232" => "solid::Uint232",
            "int240" => "solid::Int240",
            "uint240" => "solid::Uint240",
            "int248" => "solid::Int248",
            "uint248" => "solid::Uint248",
            "int256" => "solid::Int256",
            "uint256" => "solid::Uint256",

            _ => "",
        }
        .to_string()
    }
}
