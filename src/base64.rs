use base64::{Engine as _, engine::general_purpose, DecodeError};

pub struct Base64;

impl Base64 {
    pub fn encode(input: &str) -> String {
        general_purpose::STANDARD.encode(input)
    }

    pub fn decode(encoded: &str) -> Result<Vec<u8>, DecodeError>  {
        general_purpose::STANDARD.decode(encoded)
    }
}
