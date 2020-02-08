use super::{
    valid_block,
    FromBytes,
};
use crate::Result;

impl<'a> FromBytes<'a, String> for String {
    fn from_bytes(buf: &'a [u8], index: usize) -> Result<String> {
        valid_block(&buf, index * 32, (index + 1) * 32)?;
        let offset = usize::from_bytes(&buf, index)?;

        valid_block(&buf, offset, offset + 32)?;
        let len = u64::from_bytes(&buf[offset..offset + 32], 0)? as usize;

        let value = std::str::from_utf8(&buf[offset + 32..offset + 32 + len])?.into();
        Ok(value)
    }
}

impl<'a> FromBytes<'a, Vec<String>> for Vec<String> {
    fn from_bytes(buf: &'a [u8], index: usize) -> Result<Vec<String>> {
        valid_block(&buf, index * 32, (index + 1) * 32)?;
        let offset = usize::from_bytes(&buf, index)?;

        valid_block(&buf, offset, offset + 32)?;
        let len = u64::from_bytes(&buf[offset..offset + 32], 0)? as usize;

        let mut vec: Vec<String> = Vec::new();

        let offset = offset + 32;

        for i in 0..len {
            valid_block(&buf, offset + (i * 32), offset + ((i + 1) * 32))?;

            let element_offset =
                usize::from_bytes(&buf[offset + (i * 32)..offset + ((i + 1) * 32)], 0)? as usize;

            let byte_len = u64::from_bytes(
                &buf[offset + element_offset..offset + element_offset + 32],
                0,
            )? as usize;

            let value = std::str::from_utf8(
                &buf[offset + element_offset + 32..offset + element_offset + 32 + byte_len],
            )?
            .into();

            vec.push(value);
        }

        Ok(vec)
    }
}

#[cfg(test)]
mod test {
    use crate::result::SolidityResult;

    #[test]
    #[rustfmt::skip]
    fn decode_string() -> anyhow::Result<()> {
        let bytes   = hex::decode("\
0000000000000000000000000000000000000000000000000000000000000040\
0000000000000000000000000000000000000000000000000000000000000080\
000000000000000000000000000000000000000000000000000000000000000d\
72616e646f6d20737472696e6700000000000000000000000000000000000000\
0000000000000000000000000000000000000000000000000000000000000016\
776861742061626f757420616e6f74686572206f6e6500000000000000000000\
").unwrap();
        assert_eq!((&*bytes).get_param::<String>(0)?, "random string");
        assert_eq!((&*bytes).get_param::<String>(1)?, "what about another one");

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn decode_string_array() -> anyhow::Result<()> {
        let bytes   = hex::decode("\
0000000000000000000000000000000000000000000000000000000000000020\
0000000000000000000000000000000000000000000000000000000000000002\
0000000000000000000000000000000000000000000000000000000000000040\
0000000000000000000000000000000000000000000000000000000000000080\
000000000000000000000000000000000000000000000000000000000000000d\
72616e646f6d20737472696e6700000000000000000000000000000000000000\
0000000000000000000000000000000000000000000000000000000000000016\
776861742061626f757420616e6f74686572206f6e6500000000000000000000\
").unwrap();
        assert_eq!((&*bytes).get_param::<Vec<String>>(0)?, 
            vec!["random string".to_owned(), "what about another one".to_owned()]);

        Ok(())
    }
}
