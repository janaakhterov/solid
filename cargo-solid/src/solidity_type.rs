use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolidityType {
    pub internal_type: Option<String>,
    pub name: String,
    pub r#type: String,
}

impl SolidityType {
    pub fn to_rust_type(&self, lifetime: bool, nightly: bool) -> String {
        type_to_rust(self.r#type.as_str(), lifetime, nightly)
    }

    pub fn to_rust_declaration(&self, lifetime: bool, nightly: bool) -> String {
        let ty = self.to_rust_type(lifetime, nightly);
        format!("{}: {}", self.name, ty)
    }

    pub fn has_lifetime(&self) -> bool {
        self.r#type.starts_with("bytes")
            || self.r#type.starts_with("string")
            || self.r#type.starts_with("address")
            || self.r#type.starts_with("function")
    }
}

pub fn type_to_rust(ty: &str, lifetime: bool, nightly: bool) -> String {
    if ty.ends_with("[]") {
        format!(
            "Vec<{}>",
            type_to_rust(ty.split_at(ty.len() - 2).0, lifetime, nightly)
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

            "bytes1" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 1>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 1>",
                (_, false) => "solid::bytesfix::Bytes1",
            },

            "bytes2" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 2>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 2>",
                (_, false) => "solid::bytesfix::Bytes2",
            },

            "bytes3" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 3>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 3>",
                (_, false) => "solid::bytesfix::Bytes3",
            },

            "bytes4" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 4>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 4>",
                (_, false) => "solid::bytesfix::Bytes4",
            },

            "bytes5" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 5>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 5>",
                (_, false) => "solid::bytesfix::Bytes5",
            },

            "bytes6" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 6>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 6>",
                (_, false) => "solid::bytesfix::Bytes6",
            },

            "bytes7" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 7>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 7>",
                (_, false) => "solid::bytesfix::Bytes7",
            },

            "bytes8" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 8>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 8>",
                (_, false) => "solid::bytesfix::Bytes8",
            },

            "bytes9" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 9>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 9>",
                (_, false) => "solid::bytesfix::Bytes9",
            },

            "bytes10" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 10>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 10>",
                (_, false) => "solid::bytesfix::Bytes10",
            },

            "bytes11" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 11>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 11>",
                (_, false) => "solid::bytesfix::Bytes11",
            },

            "bytes12" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 12>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 12>",
                (_, false) => "solid::bytesfix::Bytes12",
            },

            "bytes13" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 13>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 13>",
                (_, false) => "solid::bytesfix::Bytes13",
            },

            "bytes14" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 14>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 14>",
                (_, false) => "solid::bytesfix::Bytes14",
            },

            "bytes15" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 15>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 15>",
                (_, false) => "solid::bytesfix::Bytes15",
            },

            "bytes16" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 16>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 16>",
                (_, false) => "solid::bytesfix::Bytes16",
            },

            "bytes17" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 17>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 17>",
                (_, false) => "solid::bytesfix::Bytes17",
            },

            "bytes18" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 18>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 18>",
                (_, false) => "solid::bytesfix::Bytes18",
            },

            "bytes19" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 19>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 19>",
                (_, false) => "solid::bytesfix::Bytes19",
            },

            "bytes20" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 20>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 20>",
                (_, false) => "solid::bytesfix::Bytes20",
            },

            "bytes21" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 21>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 21>",
                (_, false) => "solid::bytesfix::Bytes21",
            },

            "bytes22" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 22>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 22>",
                (_, false) => "solid::bytesfix::Bytes22",
            },

            "bytes23" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 23>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 23>",
                (_, false) => "solid::bytesfix::Bytes23",
            },

            "bytes24" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 24>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 24>",
                (_, false) => "solid::bytesfix::Bytes24",
            },

            "bytes25" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 25>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 25>",
                (_, false) => "solid::bytesfix::Bytes25",
            },

            "bytes26" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 26>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 26>",
                (_, false) => "solid::bytesfix::Bytes26",
            },

            "bytes27" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 27>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 27>",
                (_, false) => "solid::bytesfix::Bytes27",
            },

            "bytes28" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 28>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 28>",
                (_, false) => "solid::bytesfix::Bytes28",
            },

            "bytes29" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 29>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 29>",
                (_, false) => "solid::bytesfix::Bytes29",
            },

            "bytes30" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 30>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 30>",
                (_, false) => "solid::bytesfix::Bytes30",
            },

            "bytes31" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 31>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 31>",
                (_, false) => "solid::bytesfix::Bytes31",
            },

            "bytes32" => match (lifetime, nightly) {
                (true, true) => "solid::bytesfix::BytesFix<'a, 32>",
                (false, true) => "solid::bytesfix::BytesFix<'_, 32>",
                (_, false) => "solid::bytesfix::Bytes32",
            },

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

            "int24" => {
                if nightly {
                    "solid::int::Int<24, 3>"
                } else {
                    "solid::int::Int24"
                }
            }

            "uint24" => {
                if nightly {
                    "solid::int::Int<24, 3>"
                } else {
                    "solid::int::Uint24"
                }
            }

            "int40" => {
                if nightly {
                    "solid::int::Int<40, 5>"
                } else {
                    "solid::int::Int40"
                }
            }

            "uint40" => {
                if nightly {
                    "solid::int::Int<40, 5>"
                } else {
                    "solid::int::Uint40"
                }
            }

            "int48" => {
                if nightly {
                    "solid::int::Int<48, 6>"
                } else {
                    "solid::int::Int48"
                }
            }

            "uint48" => {
                if nightly {
                    "solid::int::Int<48, 6>"
                } else {
                    "solid::int::Uint48"
                }
            }

            "int56" => {
                if nightly {
                    "solid::int::Int<56, 7>"
                } else {
                    "solid::int::Int56"
                }
            }

            "uint56" => {
                if nightly {
                    "solid::int::Int<56, 7>"
                } else {
                    "solid::int::Uint56"
                }
            }

            "int72" => {
                if nightly {
                    "solid::int::Int<72, 9>"
                } else {
                    "solid::int::Int72"
                }
            }

            "uint72" => {
                if nightly {
                    "solid::int::Int<72, 9>"
                } else {
                    "solid::int::Uint72"
                }
            }

            "int80" => {
                if nightly {
                    "solid::int::Int<80, 10>"
                } else {
                    "solid::int::Int80"
                }
            }

            "uint80" => {
                if nightly {
                    "solid::int::Int<80, 10>"
                } else {
                    "solid::int::Uint80"
                }
            }

            "int88" => {
                if nightly {
                    "solid::int::Int<88, 11>"
                } else {
                    "solid::int::Int88"
                }
            }

            "uint88" => {
                if nightly {
                    "solid::int::Int<88, 11>"
                } else {
                    "solid::int::Uint88"
                }
            }

            "int96" => {
                if nightly {
                    "solid::int::Int<96, 12>"
                } else {
                    "solid::int::Int96"
                }
            }

            "uint96" => {
                if nightly {
                    "solid::int::Int<96, 12>"
                } else {
                    "solid::int::Uint96"
                }
            }

            "int104" => {
                if nightly {
                    "solid::int::Int<104, 13>"
                } else {
                    "solid::int::Int104"
                }
            }

            "uint104" => {
                if nightly {
                    "solid::int::Int<104, 13>"
                } else {
                    "solid::int::Uint104"
                }
            }

            "int112" => {
                if nightly {
                    "solid::int::Int<112, 14>"
                } else {
                    "solid::int::Int112"
                }
            }

            "uint112" => {
                if nightly {
                    "solid::int::Int<112, 14>"
                } else {
                    "solid::int::Uint112"
                }
            }

            "int120" => {
                if nightly {
                    "solid::int::Int<120, 15>"
                } else {
                    "solid::int::Int120"
                }
            }

            "uint120" => {
                if nightly {
                    "solid::int::Int<120, 15>"
                } else {
                    "solid::int::Uint120"
                }
            }

            "int136" => {
                if nightly {
                    "solid::int::Int<136, 17>"
                } else {
                    "solid::int::Int136"
                }
            }

            "uint136" => {
                if nightly {
                    "solid::int::Int<136, 17>"
                } else {
                    "solid::int::Uint136"
                }
            }

            "int144" => {
                if nightly {
                    "solid::int::Int<144, 18>"
                } else {
                    "solid::int::Int144"
                }
            }

            "uint144" => {
                if nightly {
                    "solid::int::Int<144, 18>"
                } else {
                    "solid::int::Uint144"
                }
            }

            "int152" => {
                if nightly {
                    "solid::int::Int<152, 19>"
                } else {
                    "solid::int::Int152"
                }
            }

            "uint152" => {
                if nightly {
                    "solid::int::Int<152, 19>"
                } else {
                    "solid::int::Uint152"
                }
            }

            "int160" => {
                if nightly {
                    "solid::int::Int<160, 20>"
                } else {
                    "solid::int::Int160"
                }
            }

            "uint160" => {
                if nightly {
                    "solid::int::Int<160, 20>"
                } else {
                    "solid::int::Uint160"
                }
            }

            "int168" => {
                if nightly {
                    "solid::int::Int<168, 21>"
                } else {
                    "solid::int::Int168"
                }
            }

            "uint168" => {
                if nightly {
                    "solid::int::Int<168, 21>"
                } else {
                    "solid::int::Uint168"
                }
            }

            "int176" => {
                if nightly {
                    "solid::int::Int<176, 22>"
                } else {
                    "solid::int::Int176"
                }
            }

            "uint176" => {
                if nightly {
                    "solid::int::Int<176, 22>"
                } else {
                    "solid::int::Uint176"
                }
            }

            "int184" => {
                if nightly {
                    "solid::int::Int<184, 23>"
                } else {
                    "solid::int::Int184"
                }
            }

            "uint184" => {
                if nightly {
                    "solid::int::Int<184, 23>"
                } else {
                    "solid::int::Uint184"
                }
            }

            "int192" => {
                if nightly {
                    "solid::int::Int<192, 24>"
                } else {
                    "solid::int::Int192"
                }
            }

            "uint192" => {
                if nightly {
                    "solid::int::Int<192, 24>"
                } else {
                    "solid::int::Uint192"
                }
            }

            "int200" => {
                if nightly {
                    "solid::int::Int<200, 25>"
                } else {
                    "solid::int::Int200"
                }
            }

            "uint200" => {
                if nightly {
                    "solid::int::Int<200, 25>"
                } else {
                    "solid::int::Uint200"
                }
            }

            "int208" => {
                if nightly {
                    "solid::int::Int<208, 26>"
                } else {
                    "solid::int::Int208"
                }
            }

            "uint208" => {
                if nightly {
                    "solid::int::Int<208, 26>"
                } else {
                    "solid::int::Uint208"
                }
            }

            "int216" => {
                if nightly {
                    "solid::int::Int<216, 27>"
                } else {
                    "solid::int::Int216"
                }
            }

            "uint216" => {
                if nightly {
                    "solid::int::Int<216, 27>"
                } else {
                    "solid::int::Uint216"
                }
            }

            "int224" => {
                if nightly {
                    "solid::int::Int<224, 28>"
                } else {
                    "solid::int::Int224"
                }
            }

            "uint224" => {
                if nightly {
                    "solid::int::Int<224, 28>"
                } else {
                    "solid::int::Uint224"
                }
            }

            "int232" => {
                if nightly {
                    "solid::int::Int<232, 29>"
                } else {
                    "solid::int::Int232"
                }
            }

            "uint232" => {
                if nightly {
                    "solid::int::Int<232, 29>"
                } else {
                    "solid::int::Uint232"
                }
            }

            "int240" => {
                if nightly {
                    "solid::int::Int<240, 30>"
                } else {
                    "solid::int::Int240"
                }
            }

            "uint240" => {
                if nightly {
                    "solid::int::Int<240, 30>"
                } else {
                    "solid::int::Uint240"
                }
            }

            "int248" => {
                if nightly {
                    "solid::int::Int<248, 31>"
                } else {
                    "solid::int::Int248"
                }
            }

            "uint248" => {
                if nightly {
                    "solid::int::Int<248, 31>"
                } else {
                    "solid::int::Uint248"
                }
            }

            "int256" => {
                if nightly {
                    "solid::int::Int<256, 32>"
                } else {
                    "solid::int::Int256"
                }
            }

            "uint256" => {
                if nightly {
                    "solid::int::Int<256, 32>"
                } else {
                    "solid::int::Uint256"
                }
            }

            _ => "",
        }
        .to_string()
    }
}
