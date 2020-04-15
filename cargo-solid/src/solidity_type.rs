use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolidityType {
    pub internal_type: Option<String>,
    pub name: String,
    pub r#type: String,
}

impl SolidityType {
    pub fn to_rust_type(&self, lifetime: bool) -> String {
        type_to_rust(self.r#type.as_str(), lifetime)
    }

    pub fn to_rust_declaration(&self, lifetime: bool) -> String {
        let ty = self.to_rust_type(lifetime);
        format!("{}: {}", self.name, ty)
    }

    pub fn has_lifetime(&self) -> bool {
        self.r#type.starts_with("bytes")
            || self.r#type.starts_with("string")
            || self.r#type.starts_with("address")
            || self.r#type.starts_with("function")
    }
}

pub fn type_to_rust(ty: &str, lifetime: bool) -> String {
    if ty.ends_with("[]") {
        format!(
            "Vec<{}>",
            type_to_rust(ty.split_at(ty.len() - 2).0, lifetime)
        )
    } else {
        match ty {
            "bytes" => {
                if lifetime {
                    "solid::Bytes<'a>"
                } else {
                    "solid::Bytes<'_>"
                }
            }
            "string" => {
                if lifetime {
                    "&'a str"
                } else {
                    "&str"
                }
            }

            "adddress" => {
                if lifetime {
                    "solid::Address<'a>"
                } else {
                    "solid::Address<'_>"
                }
            }
            "function" => {
                if lifetime {
                    "solid::Function<'a>"
                } else {
                    "solid::Function<'_>"
                }
            }

            "bytes1" => "solid::bytesfix::Bytes1",
            "bytes2" => "solid::bytesfix::Bytes2",
            "bytes3" => "solid::bytesfix::Bytes3",
            "bytes4" => "solid::bytesfix::Bytes4",
            "bytes5" => "solid::bytesfix::Bytes5",
            "bytes6" => "solid::bytesfix::Bytes6",
            "bytes7" => "solid::bytesfix::Bytes7",
            "bytes8" => "solid::bytesfix::Bytes8",
            "bytes9" => "solid::bytesfix::Bytes9",
            "bytes10" => "solid::bytesfix::Bytes10",
            "bytes11" => "solid::bytesfix::Bytes11",
            "bytes12" => "solid::bytesfix::Bytes12",
            "bytes13" => "solid::bytesfix::Bytes13",
            "bytes14" => "solid::bytesfix::Bytes14",
            "bytes15" => "solid::bytesfix::Bytes15",
            "bytes16" => "solid::bytesfix::Bytes16",
            "bytes17" => "solid::bytesfix::Bytes17",
            "bytes18" => "solid::bytesfix::Bytes18",
            "bytes19" => "solid::bytesfix::Bytes19",
            "bytes20" => "solid::bytesfix::Bytes20",
            "bytes21" => "solid::bytesfix::Bytes21",
            "bytes22" => "solid::bytesfix::Bytes22",
            "bytes23" => "solid::bytesfix::Bytes23",
            "bytes24" => "solid::bytesfix::Bytes24",
            "bytes25" => "solid::bytesfix::Bytes25",
            "bytes26" => "solid::bytesfix::Bytes26",
            "bytes27" => "solid::bytesfix::Bytes27",
            "bytes28" => "solid::bytesfix::Bytes28",
            "bytes29" => "solid::bytesfix::Bytes29",
            "bytes30" => "solid::bytesfix::Bytes30",
            "bytes31" => "solid::bytesfix::Bytes31",
            "bytes32" => "solid::bytesfix::Bytes32",

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

            "int24" => "solid::int::Int24",
            "uint24" => "solid::int::Uint24",
            "int40" => "solid::int::Int40",
            "uint40" => "solid::int::Uint40",
            "int48" => "solid::int::Int48",
            "uint48" => "solid::int::Uint48",
            "int56" => "solid::int::Int56",
            "uint56" => "solid::int::Uint56",
            "int72" => "solid::int::Int72",
            "uint72" => "solid::int::Uint72",
            "int80" => "solid::int::Int80",
            "uint80" => "solid::int::Uint80",
            "int88" => "solid::int::Int88",
            "uint88" => "solid::int::Uint88",
            "int96" => "solid::int::Int96",
            "uint96" => "solid::int::Uint96",
            "int104" => "solid::int::Int104",
            "uint104" => "solid::int::Uint104",
            "int112" => "solid::int::Int112",
            "uint112" => "solid::int::Uint112",
            "int120" => "solid::int::Int120",
            "uint120" => "solid::int::Uint120",
            "int136" => "solid::int::Int136",
            "uint136" => "solid::int::Uint136",
            "int144" => "solid::int::Int144",
            "uint144" => "solid::int::Uint144",
            "int152" => "solid::int::Int152",
            "uint152" => "solid::int::Uint152",
            "int160" => "solid::int::Int160",
            "uint160" => "solid::int::Uint160",
            "int168" => "solid::int::Int168",
            "uint168" => "solid::int::Uint168",
            "int176" => "solid::int::Int176",
            "uint176" => "solid::int::Uint176",
            "int184" => "solid::int::Int184",
            "uint184" => "solid::int::Uint184",
            "int192" => "solid::int::Int192",
            "uint192" => "solid::int::Uint192",
            "int200" => "solid::int::Int200",
            "uint200" => "solid::int::Uint200",
            "int208" => "solid::int::Int208",
            "uint208" => "solid::int::Uint208",
            "int216" => "solid::int::Int216",
            "uint216" => "solid::int::Uint216",
            "int224" => "solid::int::Int224",
            "uint224" => "solid::int::Uint224",
            "int232" => "solid::int::Int232",
            "uint232" => "solid::int::Uint232",
            "int240" => "solid::int::Int240",
            "uint240" => "solid::int::Uint240",
            "int248" => "solid::int::Int248",
            "uint248" => "solid::int::Uint248",
            "int256" => "solid::int::Int256",
            "uint256" => "solid::int::Uint256",

            _ => "",
        }
        .to_string()
    }
}
