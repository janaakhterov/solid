use super::Type;

// This macro generates all the into implementations for the various number variants.
// This is primarily helpful in the `Builder::add()` method.
#[macro_use]
macro_rules! impl_solidity_into {
    ($ty: ty, $solidity: ident) => {
        impl<'a> Into<Type<'a>> for $ty {
            fn into(self) -> Type<'a> {
                Type::$solidity(self)
            }
        }
    };
}

impl_solidity_into!(i8, I8);
impl_solidity_into!(u8, U8);
impl_solidity_into!(i16, I16);
impl_solidity_into!(u16, U16);
impl_solidity_into!(i32, I32);
impl_solidity_into!(u32, U32);
impl_solidity_into!(i64, I64);
impl_solidity_into!(u64, U64);
impl_solidity_into!(i128, I128);
impl_solidity_into!(u128, U128);
