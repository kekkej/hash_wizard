mod config;
mod hashers;
mod db;
mod formatter;

use clap::{Parser, Subcommand};
use anyhow::{Result, Context};
use db::{init_db, issue_key, rotate_key, list_keys, get_active_key};
use hashers::{Hasher};
use config::load_hashers_from_config;

/// CLI interface
#[derive(Parser)]
#[command(name = "keygen", version = "2.0")]
struct Args {
    #[command(subcommand)]
    command: Command,

    #[arg(long, default_value = "keygen.db")]
    db: String,

    #[arg(long, default_value = "hashers.toml")]
    config: String,
}

#[derive(Subcommand)]
enum Command {
    /// Issue new key (error if exists)
    Issue { user: String },

    /// Rotate (replace) key for user
    Rotate { user: String },

    /// Verify active key consistency across all hashers
    Check { user: String },

    /// Display key journal (with hashes)
    Journal {
        #[arg(long)]
        show_all: bool,

        #[arg(long)]
        output: Option<String>,
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let conn = init_db(&args.db)?;
    let hashers = load_hashers_from_config(&args.config);
    
    if hashers.is_empty() { 
        println!("Nothing to do");
        return Ok(());
    }

    let primary_hasher = &hashers[0];

    match args.command {
        Command::Issue { user } => {
            let key = generate_key();
            
            let key_hash = primary_hasher.hash(&key)
                .with_context(|| "Failed to hash generated key")?;
            
            compare_hashes(&key, &key_hash, &hashers)
                .with_context(|| "Hash inconsistency detected during key issuance")?;
            
            let id = issue_key(&conn, &user, &key, &key_hash)?;
            println!("✅ Key issued for '{}'", user);
            println!("ID: {}", id);
        }

        Command::Rotate { user } => {
            let new_key = generate_key();
            
            let key_hash = primary_hasher.hash(&new_key)
                .with_context(|| "Failed to hash new key")?;
            
            compare_hashes(&new_key, &key_hash, &hashers)
                .with_context(|| "Hash inconsistency detected during key issuance")?;
            
            let id = rotate_key(&conn, &user, &new_key, &key_hash)?;
            println!("🔄 Key rotated for '{}'", user);
            println!("New ID: {}", id);
        }

        Command::Check { user } => {
            let key = get_active_key(&conn, &user)
                .with_context(|| format!("No active key found for '{}'", user))?;

            println!("🔍 Checking key for '{}'", user);

            let mut reference: Option<String> = None;
            let mut all_match = true;

            for hasher in &hashers {
                let hash = hasher.hash(&key)
                    .with_context(|| format!("Hasher '{}' failed", hasher.name()))?;

                if let Some(ref ref_hash) = reference {
                    if !hash.eq_ignore_ascii_case(ref_hash) {
                        all_match = false;
                    }
                } else {
                    reference = Some(hash.clone());
                }

                println!("{:<12}: {}", hasher.name(), hash);
            }

            if all_match {
                println!("✅ All hashers produced identical hashes.");
            } else {
                println!("❌ Hash mismatch detected!");
                std::process::exit(1);
            }
        }

        Command::Journal { show_all, output } => {
            let entries = list_keys(&conn, show_all)?;
            let mut buffer = Vec::new();
            formatter::render_journal_table(&entries, &mut buffer)?;
            
            if let Some(path) = output {
                std::fs::write(&path, buffer)
                    .with_context(|| format!("Failed to write journal to '{}'", path))?;
                println!("✅ Journal written to '{}'", path);
            } else {
                print!("{}", String::from_utf8_lossy(&buffer));
            }
        }
    }

    Ok(())
}


fn generate_key() -> String {
    // String::from("qhyBDKoRjfpJuu4z1g5nSW6I9yFhav1isA0FrTKj8LQCLUGawg0ZGoT7sG5AUiVx")
    use rand::RngCore;
    let mut bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut bytes);
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}


pub fn compare_hashes<'a>(
    input: &str,
    canonical_hash: &str,
    hashers: &[Box<dyn Hasher + 'a>],
) -> Result<()> {
    for hasher in hashers {
        let hash = hasher.hash(input)
            .with_context(|| format!("Hasher '{}' failed", hasher.name()))?;

        if !hash.eq_ignore_ascii_case(canonical_hash) {
            anyhow::bail!(
                "Hasher '{}' mismatch: expected {}, got {}",
                hasher.name(),
                canonical_hash,
                hash
            );
        }
    }

    Ok(())
}
