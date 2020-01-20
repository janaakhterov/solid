// Trait to make `Builder::add_bytes_n()` generic over any u8 slice of size >= 1 && <= 32
pub trait SolidityBytesN {
    fn solidity_bytes_len(&self) -> usize;
}

// Generates all the impls for both [u8; size] and &[u8; size]
#[macro_use]
macro_rules! impl_solidity_bytes_n {
    ($size: expr) => {
        impl SolidityBytesN for [u8; $size] {
            fn solidity_bytes_len(&self) -> usize {
                self.len()
            }
        }

        impl SolidityBytesN for &[u8; $size] {
            fn solidity_bytes_len(&self) -> usize {
                self.len()
            }
        }
    };
}

impl_solidity_bytes_n!(1);
impl_solidity_bytes_n!(2);
impl_solidity_bytes_n!(3);
impl_solidity_bytes_n!(4);
impl_solidity_bytes_n!(5);
impl_solidity_bytes_n!(6);
impl_solidity_bytes_n!(7);
impl_solidity_bytes_n!(8);
impl_solidity_bytes_n!(9);
impl_solidity_bytes_n!(10);
impl_solidity_bytes_n!(11);
impl_solidity_bytes_n!(12);
impl_solidity_bytes_n!(13);
impl_solidity_bytes_n!(14);
impl_solidity_bytes_n!(15);
impl_solidity_bytes_n!(16);
impl_solidity_bytes_n!(17);
impl_solidity_bytes_n!(18);
impl_solidity_bytes_n!(19);
impl_solidity_bytes_n!(20);
impl_solidity_bytes_n!(21);
impl_solidity_bytes_n!(22);
impl_solidity_bytes_n!(23);
impl_solidity_bytes_n!(24);
impl_solidity_bytes_n!(25);
impl_solidity_bytes_n!(26);
impl_solidity_bytes_n!(27);
impl_solidity_bytes_n!(28);
impl_solidity_bytes_n!(29);
impl_solidity_bytes_n!(30);
impl_solidity_bytes_n!(31);
impl_solidity_bytes_n!(32);
