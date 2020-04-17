#[allow(unused_imports)]
use solid::{derive::Encode, Encode};

#[derive(Encode)]
pub struct GetDetailsNamedOutput<'a> {
    pub message_: &'a str,
    pub random_bytes_: solid::Bytes<'a>,
    pub random_bytes10_: solid::bytesfix::BytesFix<'a, 10>,
}
                        
pub struct StatefulContract;

impl StatefulContract {

    #[rustfmt::skip]
    #[allow(dead_code)]
    pub fn new(message_: &str, random_bytes_: solid::Bytes<'_>, random_bytes10_: solid::bytesfix::BytesFix<'_, 10>) -> Vec<u8> {
        solid::Builder::new()
            .push(message_)
            .push(random_bytes_)
            .push(random_bytes10_)
            .build()
    }

    #[rustfmt::skip]
    #[allow(dead_code)]
    pub fn get_details() -> Vec<u8> {
        solid::Builder::new()
            .name("getDetails")
            .build()
    }

    #[rustfmt::skip]
    #[allow(dead_code)]
    pub fn get_details_named() -> Vec<u8> {
        solid::Builder::new()
            .name("getDetailsNamed")
            .build()
    }

    #[rustfmt::skip]
    #[allow(dead_code)]
    pub fn get_message() -> Vec<u8> {
        solid::Builder::new()
            .name("getMessage")
            .build()
    }

    #[rustfmt::skip]
    #[allow(dead_code)]
    pub fn kill() -> Vec<u8> {
        solid::Builder::new()
            .name("kill")
            .build()
    }

    #[rustfmt::skip]
    #[allow(dead_code)]
    pub fn print_message(messages_: Vec<&str>) -> Vec<u8> {
        solid::Builder::new()
            .push(messages_)
            .name("printMessage")
            .build()
    }

    #[rustfmt::skip]
    #[allow(dead_code)]
    pub fn set_message(message_: &str) -> Vec<u8> {
        solid::Builder::new()
            .push(message_)
            .name("setMessage")
            .build()
    }
}
