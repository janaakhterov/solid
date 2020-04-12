use crate::{
    encode::Encode,
    into_type::IntoType,
    selector::Selector,
};

/// Function call builder
///
/// Builds a function signature along with encode parameters to can be used to
/// call a Solidity function
#[derive(Default)]
pub struct Builder<'a> {
    name: Option<&'a str>,
    selector: Selector,
    pub(super) params: Vec<(bool, Vec<u8>)>,
}

impl<'a> Builder<'a> {
    pub fn new() -> Self {
        Builder::default()
    }

    /// Set the name of the function
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    /// Push an argument to the functions argument list
    ///
    /// Each argument is used to determine the function signature.
    pub fn push<F: Encode + IntoType>(mut self, value: F) -> Self {
        self.selector = self.selector.push::<F>();
        self.params.push((F::is_dynamic(), value.encode()));
        self
    }

    /// Build the function call
    ///
    /// If a name was set the a function selector will be used. Otherwise only the
    /// parameters will be encoded. A function name must not be set if a Solidity
    /// contract constructor is to be called.
    pub fn build(self) -> Vec<u8> {
        let name_offset = if let Some(_) = self.name { 4 } else { 0 };

        let sig = if let Some(name) = self.name {
            Some(self.selector.build(name))
        } else {
            None
        };

        let total_len = self.params.iter().map(|param| param.1.len()).sum::<usize>()
            + self
                .params
                .iter()
                .map(|param| param.0)
                .filter(|&param| param == true)
                .count()
                * 32;

        let mut buf: Vec<u8> = vec![0; total_len + name_offset];

        let mut offset: usize = self.params.len() * 32 + name_offset;

        for (index, (dynamic, bytes)) in self.params.into_iter().enumerate() {
            if dynamic {
                buf[index * 32 + 24 + name_offset..(index + 1) * 32 + name_offset]
                    .copy_from_slice(&(offset as u64).to_be_bytes());
                buf[offset..offset + bytes.len()].copy_from_slice(&bytes);
                offset += bytes.len()
            } else {
                buf[index * 32 + name_offset..(index + 1) * 32 + name_offset]
                    .copy_from_slice(&bytes);
            }
        }

        if let Some(sig) = sig {
            buf[0..4].copy_from_slice(&sig)
        }

        buf
    }
}
