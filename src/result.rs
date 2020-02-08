use crate::{
    from_bytes::FromBytes,
    Result,
};

pub trait SolidityResult<'a> {
    fn get_param<F: FromBytes<'a, F>>(&self, index: usize) -> Result<F>;
}

impl<'a> SolidityResult<'a> for &'a [u8] {
    fn get_param<F: FromBytes<'a, F>>(&self, index: usize) -> Result<F> {
        F::from_bytes(&self, index)
    }
}
