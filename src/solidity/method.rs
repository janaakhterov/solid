use super::ConcreteSolidityType;
use super::SolidityType;
use crate::function::Builder;

// This macro is used to generate all the `Builder::add_*()` methods for the various number types.
#[macro_use]
macro_rules! impl_solidity_function_for_builder {
    ($ty: ty => $solidity: ident: $function: ident | $array: ident) => {
        impl<'a> Builder<'a> {
            pub fn $function(mut self, value: $ty) -> Self {
                self.params.push(ConcreteSolidityType::$solidity(
                    SolidityType::$solidity,
                    value,
                ));
                self
            }

            pub fn $array(mut self, value: &Vec<$ty>) -> Self {
                use super::SolidityArray;
                let array = value
                    .iter()
                    .map(|value| ConcreteSolidityType::$solidity(SolidityType::$solidity, *value))
                    .collect();

                self.params.push(ConcreteSolidityType::Array(
                    SolidityType::$solidity,
                    SolidityArray {
                        dimensions: 1,
                        array,
                    },
                ));
                self
            }
        }
    };
}

impl_solidity_function_for_builder!(i8 => I8: add_i8 | add_i8_array);
impl_solidity_function_for_builder!(u8 => U8: add_u8 | add_u8_array);
impl_solidity_function_for_builder!(i16 => I16: add_i16 | add_i16_array);
impl_solidity_function_for_builder!(u16 => U16 : add_u16 | add_u16_array);
impl_solidity_function_for_builder!(i32 => I32 : add_i32 | add_i32_array);
impl_solidity_function_for_builder!(u32 => U32 : add_u32 | add_u32_array);
impl_solidity_function_for_builder!(i64 => I64 : add_i64 | add_i64_array);
impl_solidity_function_for_builder!(u64 => U64 : add_u64 | add_u64_array);
impl_solidity_function_for_builder!(i128 => I128: add_i128 | add_i128_array);
impl_solidity_function_for_builder!(u128 => U128: add_u128 | add_u128_array);
impl_solidity_function_for_builder!(&'a [u8; 32] => I256: add_i256 | add_i256_array);
impl_solidity_function_for_builder!(&'a str => String: add_string | add_string_array);
impl_solidity_function_for_builder!(&'a [u8] => Bytes: add_bytes | add_bytes_array);
impl_solidity_function_for_builder!(&'a [u8; 20] => Address: add_address | add_address_array);
impl_solidity_function_for_builder!(&'a [u8; 24] => Function: add_function | add_function_array);

#[cfg(feature = "U256")]
impl_solidity_function_for_builder!(bigint::U256 => U256: add_u256 | add_u256_array);
#[cfg(not(feature = "U256"))]
impl_solidity_function_for_builder!(&'a [u8; 32] => U256: add_u256 | add_u256_array);

#[macro_use]
macro_rules! impl_solidity_bytes_n_function_for_builder {
    ($ty: ty, $size: expr, $function: ident | $array: ident) => {
        impl<'a> Builder<'a> {
            pub fn $function(mut self, value: $ty) -> Self {
                let mut bytes = [0; 32];
                bytes[0..value.as_ref().len()].copy_from_slice(value);

                self.params.push(ConcreteSolidityType::BytesN(
                    SolidityType::BytesN($size),
                    bytes,
                ));
                self
            }

            pub fn $array(mut self, value: &Vec<$ty>) -> Self {
                use super::SolidityArray;

                let array = value
                    .iter()
                    .map(|value| {
                        let mut bytes = [0; 32];
                        bytes[0..value.as_ref().len()].copy_from_slice(&value.as_ref());

                        ConcreteSolidityType::BytesN(SolidityType::BytesN($size), bytes)
                    })
                    .collect();

                self.params.push(ConcreteSolidityType::Array(
                    SolidityType::BytesN($size),
                    SolidityArray {
                        dimensions: 1,
                        array,
                    },
                ));
                self
            }
        }
    };
}

impl_solidity_bytes_n_function_for_builder!(&'a [u8; 1], 1, add_bytes_1 | add_bytes_1_array);
impl_solidity_bytes_n_function_for_builder!(&'a [u8; 2], 2, add_bytes_2 | add_bytes_2_array);
impl_solidity_bytes_n_function_for_builder!(&'a [u8; 3], 3, add_bytes_3 | add_bytes_3_array);
impl_solidity_bytes_n_function_for_builder!(&'a [u8; 4], 4, add_bytes_4 | add_bytes_4_array);
impl_solidity_bytes_n_function_for_builder!(&'a [u8; 5], 5, add_bytes_5 | add_bytes_5_array);
impl_solidity_bytes_n_function_for_builder!(&'a [u8; 6], 6, add_bytes_6 | add_bytes_6_array);
impl_solidity_bytes_n_function_for_builder!(&'a [u8; 7], 7, add_bytes_7 | add_bytes_7_array);
impl_solidity_bytes_n_function_for_builder!(&'a [u8; 8], 8, add_bytes_8 | add_bytes_8_array);
impl_solidity_bytes_n_function_for_builder!(&'a [u8; 9], 9, add_bytes_9 | add_bytes_9_array);
impl_solidity_bytes_n_function_for_builder!(&'a [u8; 10], 10, add_bytes_10 | add_bytes_10_array);
impl_solidity_bytes_n_function_for_builder!(&'a [u8; 11], 11, add_bytes_11 | add_bytes_11_array);
impl_solidity_bytes_n_function_for_builder!(&'a [u8; 12], 12, add_bytes_12 | add_bytes_12_array);
impl_solidity_bytes_n_function_for_builder!(&'a [u8; 13], 13, add_bytes_13 | add_bytes_13_array);
impl_solidity_bytes_n_function_for_builder!(&'a [u8; 14], 14, add_bytes_14 | add_bytes_14_array);
impl_solidity_bytes_n_function_for_builder!(&'a [u8; 15], 15, add_bytes_15 | add_bytes_15_array);
impl_solidity_bytes_n_function_for_builder!(&'a [u8; 16], 16, add_bytes_16 | add_bytes_16_array);
impl_solidity_bytes_n_function_for_builder!(&'a [u8; 17], 17, add_bytes_17 | add_bytes_17_array);
impl_solidity_bytes_n_function_for_builder!(&'a [u8; 18], 18, add_bytes_18 | add_bytes_18_array);
impl_solidity_bytes_n_function_for_builder!(&'a [u8; 19], 19, add_bytes_19 | add_bytes_19_array);
impl_solidity_bytes_n_function_for_builder!(&'a [u8; 20], 20, add_bytes_20 | add_bytes_20_array);
impl_solidity_bytes_n_function_for_builder!(&'a [u8; 21], 21, add_bytes_21 | add_bytes_21_array);
impl_solidity_bytes_n_function_for_builder!(&'a [u8; 22], 22, add_bytes_22 | add_bytes_22_array);
impl_solidity_bytes_n_function_for_builder!(&'a [u8; 23], 23, add_bytes_23 | add_bytes_23_array);
impl_solidity_bytes_n_function_for_builder!(&'a [u8; 24], 24, add_bytes_24 | add_bytes_24_array);
impl_solidity_bytes_n_function_for_builder!(&'a [u8; 25], 25, add_bytes_25 | add_bytes_25_array);
impl_solidity_bytes_n_function_for_builder!(&'a [u8; 26], 26, add_bytes_26 | add_bytes_26_array);
impl_solidity_bytes_n_function_for_builder!(&'a [u8; 27], 27, add_bytes_27 | add_bytes_27_array);
impl_solidity_bytes_n_function_for_builder!(&'a [u8; 28], 28, add_bytes_28 | add_bytes_28_array);
impl_solidity_bytes_n_function_for_builder!(&'a [u8; 29], 29, add_bytes_29 | add_bytes_29_array);
impl_solidity_bytes_n_function_for_builder!(&'a [u8; 30], 30, add_bytes_30 | add_bytes_30_array);
impl_solidity_bytes_n_function_for_builder!(&'a [u8; 31], 31, add_bytes_31 | add_bytes_31_array);
impl_solidity_bytes_n_function_for_builder!(&'a [u8; 32], 32, add_bytes_32 | add_bytes_32_array);
