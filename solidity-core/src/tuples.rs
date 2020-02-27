use crate::{
    decode::Decode,
    encode::Encode,
    into_type::IntoType,
};

macro_rules! impl_encode_and_into_types_for_tuples {
	  ($(($index:tt => $ident:ident) ),+) => {
        #[allow(unused)]
        impl<$($ident: Encode, )+> Encode for ($($ident, )+) {
            fn encode(&self) -> Vec<u8> {
                let len = self.required_len();

                let mut buf: Vec<u8> = vec![0; len as usize];

                let mut count = 0usize;

                $(
                    count += if $ident::is_dynamic() { 1 } else { 1 };
                )+

                let mut offset: usize = count * 32;

                let mut index = 0;

                $(
                    let bytes = self.$index.encode();

                    if $ident::is_dynamic() {
                        buf[index * 32 + 24..(index + 1) * 32]
                            .copy_from_slice(&(offset as u64).to_be_bytes());
                        buf[offset..offset + bytes.len()]
                            .copy_from_slice(&bytes);
                        offset += bytes.len()
                    } else {
                        buf[index * 32..(index + 1) * 32]
                            .copy_from_slice(&bytes);
                    }

                    index += 1;
                )+

                buf
            }

            fn required_len(&self) -> u64 {
                let mut len = 0u64;

                $(
                    len += self.$index.required_len();
                    len += if $ident::is_dynamic() {
                        32
                    } else {
                        0
                    };
                )+

                len
            }

            fn is_dynamic() -> bool {
                true
            }
        }

        impl<$($ident: IntoType, )+> IntoType for ($($ident,) +) {
            fn into_type() -> String {
                let mut ty = Vec::new();
                $(
                    ty.push($ident::into_type());
                )+

                format!("({})", ty.join(","))
            }
        }

        #[allow(unused)]
        impl<'a, $($ident: Encode + Decode<'a>, )+> Decode<'a> for ($($ident,) +)
        {
            fn decode(buf: &'a [u8]) -> Self {
                let mut index = 0;


                (
                    $(
                        {
                            let value = if $ident::is_dynamic() {
                                $ident::decode(&buf[index * 32..(index + 1) * 32])
                            } else {
                                let offset = u64::decode(&buf[index * 32..(index + 1) * 32]);
                                $ident::decode(&buf[offset as usize..])
                            };
                            index += 1;
                            value
                        },
                    )+
                )
            }
        }
	  };
}

impl_encode_and_into_types_for_tuples!(
    (0 => T0),
    (1 => T1)
);
impl_encode_and_into_types_for_tuples!(
    (0 => T0),
    (1 => T1),
    (2 => T2)
);

impl_encode_and_into_types_for_tuples!(
    (0 => T0),
    (1 => T1),
    (2 => T2),
    (3 => T3)
);

impl_encode_and_into_types_for_tuples!(
    (0 => T0),
    (1 => T1),
    (2 => T2),
    (3 => T3),
    (4 => T4)
);

impl_encode_and_into_types_for_tuples!(
    (0 => T0),
    (1 => T1),
    (2 => T2),
    (3 => T3),
    (4 => T4),
    (5 => T5)
);

impl_encode_and_into_types_for_tuples!(
    (0 => T0),
    (1 => T1),
    (2 => T2),
    (3 => T3),
    (4 => T4),
    (5 => T5),
    (6 => T6)
);
