use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HasherConfigFile {
    pub hasher: Vec<HasherEntry>,
}

#[derive(Debug, Deserialize)]
pub struct HasherEntry {
    pub name: String,
    #[serde(rename = "type")]
    pub hasher_type: HasherType,
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub algorithm: HashAlgorithm,
    pub input_mode: Option<InputMode>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HasherType {
    Builtin,
    External,
}

use crate::hashers::{Hasher, ExternalHasher, HashAlgorithm, RustSha256Hasher};

pub fn load_hashers_from_config(path: &str) -> Vec<Box<dyn Hasher>> {
    let content = std::fs::read_to_string(path)
        .expect(format!("Failed to read config in {}", path)
            .as_str());
    let config: HasherConfigFile = toml::from_str(&content)
        .expect("Failed to parse TOML");

    config
        .hasher
        .into_iter()
        .map(|entry| match entry.hasher_type {
            HasherType::Builtin => match entry.name.as_str() {
                "Rust" => create_rust_hasher(entry.algorithm),
                _ => panic!("Unknown builtin hasher: {}", entry.name),
            },
            HasherType::External => {
                let cmd = entry.command.expect("Missing command");
                let args = entry.args.unwrap_or_default();
                Box::new(ExternalHasher {
                    name: entry.name,
                    command: cmd,
                    args,
                    algorithm: entry.algorithm,
                    input_mode: entry.input_mode.unwrap_or(InputMode::Arg),
                }) as Box<dyn Hasher>
            }
        })
        .collect()
}

fn create_rust_hasher(alg: HashAlgorithm) -> Box<dyn Hasher> {
    match alg {
        HashAlgorithm::Sha256 => { 
            Box::new(RustSha256Hasher)
        }
        HashAlgorithm::Sha1 => panic!("Unknown rust HashAlgorithm: {}", alg.name()),
        HashAlgorithm::Sha512 => panic!("Unknown rust HashAlgorithm: {}", alg.name()),
        HashAlgorithm::Md5 => panic!("Unknown rust HashAlgorithm: {}", alg.name())
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InputMode {
    Arg,
    Stdin,
    File,
}