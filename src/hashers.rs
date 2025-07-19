use std::io::Write;
use std::process::{Command, Stdio};
use serde::{Deserialize, Deserializer};
use anyhow::{Context, Result};
use crate::config::InputMode;

#[derive(Debug, Clone, Copy)]
pub enum HashAlgorithm {
    Sha256,
    Sha1,
    Sha512,
    Md5,
}

impl<'de> Deserialize<'de> for HashAlgorithm {
    fn deserialize<D>(deserializer: D) -> Result<HashAlgorithm, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let normalized = s.trim().to_lowercase().replace("-", "");

        match normalized.as_str() {
            "sha256" => Ok(HashAlgorithm::Sha256),
            "sha1" => Ok(HashAlgorithm::Sha1),
            "sha512" => Ok(HashAlgorithm::Sha512),
            "md5" => Ok(HashAlgorithm::Md5),
            other => Err(serde::de::Error::custom(format!(
                "Unknown hash algorithm: '{}'", other
            ))),
        }
    }
}


impl HashAlgorithm {
    pub fn name(&self) -> &'static str {
        match self {
            HashAlgorithm::Sha256 => "sha256",
            HashAlgorithm::Sha1 => "sha1",
            HashAlgorithm::Sha512 => "sha512",
            HashAlgorithm::Md5 => "md5",
        }
    }

    pub fn regex(&self) -> regex::Regex {
        let pattern = match self {
            HashAlgorithm::Sha256 => r"(?i)\b[a-f0-9]{64}\b",
            HashAlgorithm::Sha1   => r"(?i)\b[a-f0-9]{40}\b",
            HashAlgorithm::Sha512 => r"(?i)\b[a-f0-9]{128}\b",
            HashAlgorithm::Md5    => r"(?i)\b[a-f0-9]{32}\b",
        };
        regex::Regex::new(pattern).unwrap()
    }
}



pub trait Hasher {
    fn name(&self) -> &str;
    fn hash(&self, input: &str) -> Result<String>;
}

pub struct RustSha256Hasher;

impl Hasher for RustSha256Hasher {
    fn name(&self) -> &str {
        "Rust"
    }

    fn hash(&self, input: &str) -> Result<String> {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        Ok(format!("{:x}", hasher.finalize()))
    }
}

pub struct ExternalHasher {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub algorithm: HashAlgorithm,
    pub input_mode: InputMode,
}
impl Hasher for ExternalHasher {
    fn name(&self) -> &str {
        &self.name
    }

    fn hash(&self, input: &str) -> Result<String> {
        let algorithm = self.algorithm;
        match self.input_mode {
            InputMode::Arg => {
                let output = Command::new(&self.command)
                    .args(&self.args)
                    .arg(input)
                    .output()
                    .with_context(|| format!("Failed to run '{}'", self.command))?;

                process_output_with_algorithm(output, algorithm)
            }

            InputMode::Stdin => {
                let mut process = Command::new(&self.command)
                    .args(&self.args)
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()
                    .with_context(|| format!("Failed to spawn '{}'", self.command))?;

                process.stdin.as_mut()
                    .context("Failed to open stdin")?
                    .write_all(input.as_bytes())
                    .context("Failed to write to stdin")?;

                let output = process
                    .wait_with_output()
                    .context("Failed to read process output")?;
                
                process_output_with_algorithm(output, algorithm)
            }

            InputMode::File => {
                use std::io::Write;
                use tempfile::NamedTempFile;

                let mut temp = NamedTempFile::new()
                    .context("Failed to create temporary file")?;

                write!(temp, "{}", input)
                    .context("Failed to write input to temporary file")?;

                let path = temp.path().to_str()
                    .context("Failed to get temporary file path")?;

                let substituted_args: Vec<String> = self
                    .args
                    .iter()
                    .map(|a| a.replace("{{INPUT_FILE}}", path))
                    .collect();

                let output = Command::new(&self.command)
                    .args(&substituted_args)
                    .output()
                    .with_context(|| format!("Failed to run '{}'", self.command))?;

                process_output_with_algorithm(output, algorithm)
            }
        }
    }

}

fn process_output_with_algorithm(
    output: std::process::Output,
    algorithm: HashAlgorithm,
) -> Result<String> {
    if !output.status.success() {
        let unsuccess_error = String::from_utf8_lossy(&output.stderr).to_string();
        return Err(anyhow::anyhow!(unsuccess_error));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let re = algorithm.regex();
    if let Some(mat) = re.find(&stdout) {
        Ok(mat.as_str().to_string())
    } else {
        Err(anyhow::anyhow!("No valid {} hash found in output", algorithm.name()))
    }
}