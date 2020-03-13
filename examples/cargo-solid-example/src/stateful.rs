
pub struct StatefulContract;

impl StatefulContract {

    pub fn new(message_: &str) -> Vec<u8> {
        solid::Builder::new()
            .push(message_)
            
            .build()
    }

    pub fn getMessage() -> Vec<u8> {
        solid::Builder::new()
            
            .name("getMessage")
            .build()
    }

    pub fn kill() -> Vec<u8> {
        solid::Builder::new()
            
            .name("kill")
            .build()
    }

    pub fn printMessage(messages_: Vec<&str>) -> Vec<u8> {
        solid::Builder::new()
            .push(messages_)
            .name("printMessage")
            .build()
    }

    pub fn setMessage(message_: &str) -> Vec<u8> {
        solid::Builder::new()
            .push(message_)
            .name("setMessage")
            .build()
    }
}
