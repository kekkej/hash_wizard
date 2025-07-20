use anyhow::{Result, Context};
use std::fs::write;

pub enum OutputTarget {
    Console,
    File(String),
}

pub struct OutputController {
    pub target: OutputTarget,
}

impl OutputController {
    pub fn output(&self, content: &str) -> Result<()> {
        match &self.target {
            OutputTarget::Console => {
                println!("{}", content);
            }
            OutputTarget::File(path) => {
                write(path, content)
                    .with_context(|| format!("Failed to write output to '{}'", path))?;
                println!("✅ Output written to '{}'", path);
            }
        }
        Ok(())
    }
}
