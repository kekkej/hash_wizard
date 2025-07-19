mod hashers;
mod config;

use std::string::String;
use clap::Parser;
use crate::config::load_hashers_from_config;
use crate::hashers::{Hasher, RustSha256Hasher};


#[derive(Parser)]
#[command(name = "KeyGen CLI", version = "1.0", author = "Ты")]
struct Args {
    #[arg(short, long)]
    user: String,

    /// console, sql, vault
    #[arg(short, long, default_value = "console")]
    output: String,

    #[arg(long, default_value = "users")]
    table: String,

    #[arg(long)]
    check: bool,

    #[arg(long, default_value = "hashers.toml")]
    config: String,
}

fn main() {
    let args = Args::parse();

    let hashers = load_hashers_from_config(&args.config);
    let key = generate_key();

    println!("🔐 Generated key for '{}'", args.user);

    let results = run_hashers(&hashers, &key);
    let res = RustSha256Hasher.hash(&key);
    match res {
        Ok(v) => {println!("result = {}", v);}
        Err(_) => {println!("error")}
    }

    if args.check {
        compare_hashes(&results);
    }
    
    handle_output(&args.output, &args.user, &key, &args.table);
}

fn generate_key() -> String {
    // String::from("qhyBDKoRjfpJuu4z1g5nSW6I9yFhav1isA0FrTKj8LQCLUGawg0ZGoT7sG5AUiVx")
    use rand::RngCore;
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn run_hashers<'a>(
    hashers: &'a [Box<dyn Hasher + 'a>],
    input: &str
) -> Vec<(&'a str, Result<String, String>)> {
    hashers
        .iter()
        .map(|h| (h.name(), h.hash(input)))
        .collect()
}

fn compare_hashes(results: &[(&str, Result<String, String>)]) {
    println!("🧪 Comparing hashes:\n");

    let reference = results.iter()
        .find_map(|(name, res)| res.as_ref().ok().map(|hash| (hash, *name)));

    if let Some((ref_hash, _ref_name)) = reference {
        for (name, result) in results {
            match result {
                Ok(hash) => {
                    let status = if hash.eq_ignore_ascii_case(ref_hash) { "✅" } else { "❌" };
                    println!("{:<10}: {} {}", name, hash, status);
                }
                Err(e) => {
                    println!("{:<10}: ❌ ERROR: {}", name, e);
                }
            }
        }
    } else {
        println!("❌ No valid hash to compare with");
    }
}

fn handle_output(mode: &str, user: &str, key: &str, table: &str) {
    match mode {
        "console" => {
            println!("👤 User: {}", user);
            println!("🔑 Key: {}", key);
        }

        "sql" => {
            println!(
                "📄 SQL:\nINSERT INTO {} (name, key, is_active) VALUES ('{}', '{}', true);",
                table, user, key
            );
        }

        "vault" => {
            if let Err(e) = try_write_to_vault(user, key) {
                eprintln!("❌ Vault error: {}", e);
                println!("🔑 Key: {}", key);
            } else {
                println!("✅ Key written to vault for user '{}'", user);
            }
        }

        _ => {
            eprintln!("❌ Unknown output mode: {}", mode);
        }
    }
}


fn try_write_to_vault(user: &str, key: &str) -> Result<(), String> {

    if user.contains("fail") {
        return Err("vault failure".into());
    }

    println!("Writing to vault: user={}, key={}", user, key);
    Ok(())
}
