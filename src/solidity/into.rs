use super::ConcreteSolidityType;
use super::SolidityType;

pub trait IntoType<'a> {
    fn into_type(self) -> ConcreteSolidityType<'a>;
}

pub trait IntoSolidityType {
    fn into_solidity_type(&self) -> SolidityType;
}

pub trait IntoVecType<'a> {
    fn into_vec_type(&self) -> Vec<ConcreteSolidityType<'a>>;
}

// This macro generates all the into implementations for the various number variants.
// This is primarily helpful in the `Builder::add()` method.
#[macro_use]
macro_rules! impl_solidity_into {
    ($ty: ty, $solidity: ident) => {
        impl<'a> IntoType<'a> for $ty {
            fn into_type(self) -> ConcreteSolidityType<'a> {
                ConcreteSolidityType::$solidity(SolidityType::$solidity, self)
            }
        }

        impl<'a> IntoType<'a> for &$ty {
            fn into_type(self) -> ConcreteSolidityType<'a> {
                ConcreteSolidityType::$solidity(SolidityType::$solidity, *self)
            }
        }

        impl IntoSolidityType for Vec<$ty> {
            fn into_solidity_type(&self) -> SolidityType {
                SolidityType::$solidity
            }
        }

        impl IntoSolidityType for &Vec<$ty> {
            fn into_solidity_type(&self) -> SolidityType {
                SolidityType::$solidity
            }
        }

        impl IntoSolidityType for &[$ty] {
            fn into_solidity_type(&self) -> SolidityType {
                SolidityType::$solidity
            }
        }

        impl<'a> IntoVecType<'a> for &Vec<$ty> {
            fn into_vec_type(&self) -> Vec<ConcreteSolidityType<'a>> {
                self.iter()
                    .map(|value| ConcreteSolidityType::$solidity(SolidityType::$solidity, *value))
                    .collect()
            }
        }

        impl<'a> IntoVecType<'a> for Vec<$ty> {
            fn into_vec_type(&self) -> Vec<ConcreteSolidityType<'a>> {
                self.iter()
                    .map(|value| ConcreteSolidityType::$solidity(SolidityType::$solidity, *value))
                    .collect()
            }
        }

        impl<'a> IntoVecType<'a> for &[$ty] {
            fn into_vec_type(&self) -> Vec<ConcreteSolidityType<'a>> {
                self.iter()
                    .map(|value| ConcreteSolidityType::$solidity(SolidityType::$solidity, *value))
                    .collect()
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

#[cfg(feature = "U256")]
impl_solidity_into!(bigint::U256, U256);
