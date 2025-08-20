use crate::errors::Result;
use base64::Engine;
use base64::alphabet;
use base64::engine;
use rand::SeedableRng;
use rand::seq::SliceRandom;
use thiserror::Error;

const BASE64_ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

#[derive(Debug, Error)]
pub enum Base64Error {
    #[error("BASE64解码失败，无效的BASE64字符串")]
    DecodeError,

    #[error("BASE64初始化失败")]
    InitError,
}

pub struct Base64 {
    engine: engine::GeneralPurpose,
}

impl Base64 {
    pub fn new(password: &str) -> Result<Base64> {
        let mut chars: Vec<char> = BASE64_ALPHABET.chars().collect();
        let seed: u64 = password.chars().map(|c| c as u64).sum();
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        chars.shuffle(&mut rng);
        let new_char = chars.into_iter().collect::<String>();
        let alphabet = alphabet::Alphabet::new(&new_char).map_err(|_| Base64Error::InitError)?;

        let config = engine::GeneralPurposeConfig::new()
            .with_decode_allow_trailing_bits(true)
            .with_encode_padding(true)
            .with_decode_padding_mode(engine::DecodePaddingMode::Indifferent);

        Ok(Self {
            engine: engine::GeneralPurpose::new(&alphabet, config),
        })
    }

    pub fn encode(&self, text: &str) -> String {
        self.engine.encode(text)
    }

    pub fn decode(&self, text: &str) -> Result<String> {
        let decoded = self
            .engine
            .decode(text)
            .map_err(|_| Base64Error::DecodeError)?;

        String::from_utf8(decoded).map_err(|_| Base64Error::DecodeError.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use setting::DEBUG_BASE_PATH;
    use setting::DEBUG_CONFIG_NAME;
    use setting::DEBUG_README_NAME;
    use setting::DEBUG_UPDATE_NAME;
    use setting::MAIN_PKG_NAME;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_base64() {
        let path = Path::new(DEBUG_BASE_PATH).join(DEBUG_UPDATE_NAME);
        let data = fs::read_to_string(path).unwrap();
        let json: Vec<Value> = serde_json::from_str(&data).unwrap();
        let config_json = json.get(0).unwrap();
        let readme_version = config_json
            .get("readme")
            .unwrap()
            .get("version")
            .unwrap()
            .as_str()
            .unwrap();
        let config_version = config_json
            .get("config")
            .unwrap()
            .get("version")
            .unwrap()
            .as_str()
            .unwrap();

        read_and_save(DEBUG_UPDATE_NAME, MAIN_PKG_NAME,config_version, true);
        read_and_save(DEBUG_README_NAME, readme_version,config_version, false);
        read_and_save(DEBUG_CONFIG_NAME, config_version,config_version, true);
    }

    fn read_and_save(name: &str, version: &str, version2: &str, is_json: bool) {

        let path = Path::new(DEBUG_BASE_PATH).join(name.to_lowercase());
        let base_name = name.split(".").collect::<Vec<&str>>()[0];
        let new_name = format!("{}.zip", &base_name);
        let new_path = Path::new(DEBUG_BASE_PATH)
            .join(".cargo")
            .join(&new_name.to_lowercase());
        let bak_version = if  MAIN_PKG_NAME == version {
            version2
        }else{
            version
        };

        let bak_path = Path::new(DEBUG_BASE_PATH)
            .join("bak")
            .join(format!("{}-{}.bak", base_name, bak_version).to_lowercase());
        let decode_path = Path::new(DEBUG_BASE_PATH)
            .join(".cargo")
            .join(&base_name.to_lowercase()); 
        let data = fs::read_to_string(&path).unwrap();
        let json = if is_json {
            let j: Value = serde_json::from_str(&data).unwrap();
            j.to_string()
        } else {
            data
        };
        let base64 =  Base64::new(version).unwrap();
        let json_str = json.to_string();
        let edata: String =base64.encode(&json_str);
        fs::write(&bak_path, json_str).unwrap();
        fs::write(&new_path, edata).unwrap();
        let edata = fs::read_to_string(&new_path).unwrap();
        let ddata = base64.decode(&edata).unwrap();
        fs::write(&decode_path, ddata).unwrap();
    }
}
