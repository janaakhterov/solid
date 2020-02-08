use super::{
    valid_block,
    FromBytes,
};
use crate::Result;

impl<'a> FromBytes<'a, Vec<u8>> for Vec<u8> {
    fn from_bytes(buf: &'a [u8], index: usize) -> Result<Vec<u8>> {
        valid_block(&buf, index * 32, (index + 1) * 32)?;
        let offset = usize::from_bytes(&buf, index)?;

        valid_block(&buf, offset, offset + 32)?;
        let len = u64::from_bytes(&buf[offset..offset + 32], 0)? as usize;

        let value = buf[offset + 32..offset + 32 + len].into();
        Ok(value)
    }
}

impl<'a> FromBytes<'a, Vec<Vec<u8>>> for Vec<Vec<u8>> {
    fn from_bytes(buf: &'a [u8], index: usize) -> Result<Vec<Vec<u8>>> {
        valid_block(&buf, index * 32, (index + 1) * 32)?;
        let offset = usize::from_bytes(&buf, index)?;

        valid_block(&buf, offset, offset + 32)?;
        let len = u64::from_bytes(&buf[offset..offset + 32], 0)? as usize;

        let mut vec: Vec<Vec<u8>> = Vec::new();

        let offset = offset + 32;

        for i in 0..len {
            valid_block(&buf, offset + (i * 32), offset + ((i + 1) * 32))?;

            let element_offset =
                usize::from_bytes(&buf[offset + (i * 32)..offset + ((i + 1) * 32)], 0)? as usize;

            let byte_len = u64::from_bytes(
                &buf[offset + element_offset..offset + element_offset + 32],
                0,
            )? as usize;

            let value =
                buf[offset + element_offset + 32..offset + element_offset + 32 + byte_len].into();

            vec.push(value);
        }

        Ok(vec)
    }
}
