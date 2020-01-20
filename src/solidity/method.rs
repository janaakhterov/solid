use crate::function::Builder;

// This macro is used to generate all the `Builder::add_*()` methods for the various number types.
#[macro_use]
macro_rules! impl_solidity_function_for_builder {
    ($function: ident, $ty: ty) => {
        impl<'a> Builder<'a> {
            pub fn $function(self, value: $ty) -> Self {
                self.add(value)
            }
        }
    };
}

impl_solidity_function_for_builder!(add_i8, i8);
impl_solidity_function_for_builder!(add_u8, u8);
impl_solidity_function_for_builder!(add_i16, i16);
impl_solidity_function_for_builder!(add_u16, u16);
impl_solidity_function_for_builder!(add_i32, i32);
impl_solidity_function_for_builder!(add_u32, u32);
impl_solidity_function_for_builder!(add_i64, i64);
impl_solidity_function_for_builder!(add_u64, u64);
impl_solidity_function_for_builder!(add_i128, i128);
impl_solidity_function_for_builder!(add_u128, u128);

#[cfg(feature = "U256")]
impl_solidity_into!(bigint::U256, U256);
